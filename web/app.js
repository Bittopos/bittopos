(function () {
  "use strict";
  const KEY = "bittopos.web.prototype.v1";
  const $ = (id) => document.getElementById(id);
  const state = { resources: {}, workspace: { content: { instances: [] } }, selected: null, expanded: new Set(), search: "", view: "tree" };
  const tree = $("resource-tree"), structure = $("structure-panel"), map = $("map-panel"), details = $("details-content");
  const dialog = $("resource-dialog"), help = $("help-dialog");

  function uuid() {
    if (crypto && crypto.randomUUID) return crypto.randomUUID();
    return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, c => { const r = Math.random() * 16 | 0; return (c === "x" ? r : (r & 3 | 8)).toString(16); });
  }
  function esc(v) { return String(v == null ? "" : v).replace(/[&<>"']/g, c => ({ "&":"&amp;", "<":"&lt;", ">":"&gt;", '"':"&quot;", "'":"&#39;" }[c])); }
  function el(tag, cls, text) { const n = document.createElement(tag); if (cls) n.className = cls; if (text !== undefined) n.textContent = text; return n; }
  function quoteToml(v) { return '"' + String(v == null ? "" : v).replace(/\\/g,"\\\\").replace(/"/g,'\\"').replace(/\n/g,"\\n").replace(/\r/g,"\\r").replace(/\t/g,"\\t") + '"'; }
  function tomlString(v) { const s = v.trim(); if (s.startsWith('"') && s.endsWith('"')) { try { return JSON.parse(s); } catch (_) { return s.slice(1,-1); } } return s; }
  function tomlArray(v) {
    const s = v.trim(); if (!s.startsWith("[") || !s.endsWith("]")) return [];
    const inside = s.slice(1,-1); let parts = [], token = "", quoted = false, escaped = false;
    for (const c of inside) { if (escaped) { token += c; escaped = false; continue; } if (c === "\\" && quoted) { token += c; escaped = true; continue; } if (c === '"') quoted = !quoted; if (c === "," && !quoted) { if (token.trim()) parts.push(tomlString(token)); token = ""; } else token += c; }
    if (token.trim()) parts.push(tomlString(token)); return parts;
  }
  function parseToml(text) {
    const out = { id:"", title:"", description:null, tags:[] }; let section = "";
    text.split(/\r?\n/).forEach(line => {
      const s = line.trim(); if (!s || s.startsWith("#")) return;
      const h = s.match(/^\[([^\]]+)\]$/); if (h) { section = h[1]; return; }
      const p = s.match(/^([A-Za-z_][\w-]*)\s*=\s*(.*)$/); if (!p) return;
      if (!section && p[1] === "id") out.id = tomlString(p[2]);
      if (section === "info" && p[1] === "title") out.title = tomlString(p[2]);
      if (section === "info" && p[1] === "description") out.description = tomlString(p[2]);
      if (section === "info" && p[1] === "tags") out.tags = tomlArray(p[2]);
    });
    if (!out.id || !out.title) throw new Error("El TOML debe incluir id e info.title.");
    return { id:out.id, info:{title:out.title, description:out.description, tags:out.tags} };
  }
  function normalizeResource(r) {
    if (!r || typeof r.id !== "string" || !r.info || typeof r.info.title !== "string") throw new Error("Recurso inválido: se esperaba id e info.title.");
    return { id:r.id, info:{title:r.info.title, description:typeof r.info.description === "string" ? r.info.description : null, tags:Array.isArray(r.info.tags) ? r.info.tags.filter(t => typeof t === "string") : []} };
  }
  function normalizeInstance(i, seen) {
    if (!i || typeof i.id !== "string" || !i.source || typeof i.source.id !== "string") throw new Error("Cada instancia debe contener id y source.id.");
    if (seen.has(i.id)) throw new Error("El workspace contiene un ID de instancia duplicado.");
    seen.add(i.id);
    const kids = i.content && Array.isArray(i.content.instances) ? i.content.instances.map(child => normalizeInstance(child, seen)) : [];
    const allocation = i.semantic && Array.isArray(i.semantic.allocation) ? i.semantic.allocation : [];
    if (kids.length && !i.semantic) throw new Error("Falta semantic.allocation en una instancia que contiene hijos.");
    if (kids.length && (allocation.length !== kids.length || allocation.some((id, n) => id !== kids[n].id))) throw new Error("semantic.allocation no coincide con content.instances en " + i.id);
    const out = { id:i.id, source:{id:i.source.id} };
    if (kids.length) { out.semantic = {allocation:kids.map(k => k.id)}; out.content = {instances:kids}; }
    return out;
  }
  function normalizeWorkspace(w) {
    if (!w || !w.content || !Array.isArray(w.content.instances)) throw new Error("Se esperaba content.instances en el JSON.");
    const seen = new Set(); return {content:{instances:w.content.instances.map(i => normalizeInstance(i,seen))}};
  }
  function flat() {
    const out = [];
    function visit(list, depth, parent) { list.forEach(i => { out.push({i,depth,parent}); visit(i.content ? i.content.instances : [], depth+1, i.id); }); }
    visit(state.workspace.content.instances,0,null); return out;
  }
  function find(id) { const x = flat().find(x => x.i.id === id); return x ? x.i : null; }
  function parentId(id) { const x = flat().find(x => x.i.id === id); return x ? x.parent : null; }
  function depth(id) { const x = flat().find(x => x.i.id === id); return x ? x.depth : 0; }
  function children(i) { return i ? (i.content ? i.content.instances : []) : state.workspace.content.instances; }
  function res(i) { return i ? state.resources[i.source.id] || null : null; }
  function title(i) { const r = res(i); return r ? r.info.title : "Recurso sin importar"; }
  function notify(message, kind) { const n = el("div","toast " + (kind || ""),message); $("toast-region").appendChild(n); setTimeout(() => n.remove(),4500); }
  function save() {
    try { localStorage.setItem(KEY,JSON.stringify({resources:state.resources,workspace:state.workspace,selected:state.selected,expanded:Array.from(state.expanded)})); }
    catch (_) { notify("No se pudo guardar en este navegador. Exporta los cambios.","error"); }
  }
  function load() {
    try {
      const s = JSON.parse(localStorage.getItem(KEY) || "null"); if (!s) return;
      const rs = {}; Object.values(s.resources || {}).forEach(r => { const n=normalizeResource(r); rs[n.id]=n; });
      state.resources=rs; state.workspace=normalizeWorkspace(s.workspace || {content:{instances:[]}});
      state.selected=s.selected && findInWorkspace(state.workspace,s.selected) ? s.selected : null;
      state.expanded=new Set(Array.isArray(s.expanded)?s.expanded:[]);
    } catch (e) { state.resources={}; state.workspace={content:{instances:[]}}; state.selected=null; state.expanded=new Set(); notify("No se pudieron leer los datos guardados; se inició un espacio vacío. "+e.message,"error"); }
  }
  function findInWorkspace(w,id) {
    let found=false; function visit(list){list.forEach(i=>{if(i.id===id)found=true; if(i.content)visit(i.content.instances);});}
    visit(w.content.instances); return found;
  }
  function matches(i,q) { const r=res(i); return ((r?r.info.title:"") + " " + (r?r.info.description||"":"") + " " + (r?r.info.tags.join(" "):"") + " " + i.id + " " + i.source.id).toLowerCase().includes(q); }
  function subtreeMatches(i,q) { return matches(i,q) || children(i).some(k=>subtreeMatches(k,q)); }

  function renderTreeRow(i, depthLevel, parent) {
    const kids=children(i), selected=state.selected===i.id;
    const row=el("div","tree-row"+(selected?" selected":"")); row.style.paddingLeft=(7+depthLevel*13)+"px"; row.setAttribute("role","treeitem"); row.setAttribute("aria-selected",String(selected));
    const toggle=el("span","tree-chevron",kids.length?(state.expanded.has(i.id)||state.search?"⌄":"›"):"·");
    row.append(toggle,el("span","tree-symbol","◇"),el("span","tree-title",title(i)));
    if(kids.length) row.appendChild(el("span","tree-count",String(kids.length)));
    row.addEventListener("click",e=>{e.stopPropagation();state.selected=i.id;if(kids.length&&(e.target===toggle||!state.expanded.has(i.id))){if(state.expanded.has(i.id))state.expanded.delete(i.id);else state.expanded.add(i.id);}renderAndSave();});
    parent.appendChild(row);
    if(kids.length&&(state.expanded.has(i.id)||state.search)) kids.forEach(k=>{if(!state.search||subtreeMatches(k,state.search))renderTreeRow(k,depthLevel+1,parent);});
  }
  function renderTree() {
    tree.innerHTML=""; const q=state.search.trim().toLowerCase();
    const roots=state.workspace.content.instances.filter(i=>!q||subtreeMatches(i,q));
    if(!roots.length){tree.appendChild(el("div","tree-empty",q?"No hay coincidencias.":"Todavía no hay instancias."));return;}
    roots.forEach(i=>renderTreeRow(i,0,tree));
  }
  function detailRow(parent,label,value,idValue) {
    const row=el("div","detail-row"); row.appendChild(el("span","",label)); row.appendChild(el("strong",idValue?"id-value":"",value)); parent.appendChild(row);
  }
  function action(parent,icon,label,fn,danger) {
    const b=el("button","detail-action"+(danger?" danger":"")); b.append(el("span","",icon),el("span","",label)); b.addEventListener("click",fn); parent.appendChild(b);
  }
  function renderDetails() {
    details.innerHTML=""; const i=find(state.selected);
    if(!i){const box=el("div","inspector-empty");box.innerHTML='<div class="empty-orbit">◇</div><h3>Inspector de recursos</h3><p>Selecciona una instancia para consultar su definición, identidad y ubicación en el workspace.</p>';details.appendChild(box);return;}
    const r=res(i), p=parentId(i.id);
    details.appendChild(el("div","inspector-icon","◇")); details.appendChild(el("span","type-pill",r?"Definición vinculada":"Definición no importada"));
    details.appendChild(el("h2","",r?r.info.title:"Recurso sin importar"));
    details.appendChild(el("p","details-description",r?(r.info.description||"Este recurso todavía no tiene descripción."):"Importa el TOML de esta definición para consultar sus metadatos."));
    if(r){const sec=el("section","detail-section");sec.appendChild(el("h3","","ETIQUETAS"));const tags=el("div","tag-list");if(r.info.tags.length)r.info.tags.forEach(t=>tags.appendChild(el("span","tag",t)));else tags.appendChild(el("span","tag","Sin etiquetas"));sec.appendChild(tags);details.appendChild(sec);}
    const ids=el("section","detail-section");ids.appendChild(el("h3","","IDENTIDAD"));detailRow(ids,"ID de instancia",i.id,true);detailRow(ids,"ID del recurso",i.source.id,true);detailRow(ids,"Instancia padre",p?title(find(p)):"Raíz",false);detailRow(ids,"Instancias hijas",String(children(i).length),false);details.appendChild(ids);
    const actions=el("div","detail-actions");
    if(r){action(actions,"✎","Editar definición",()=>editResource(r.id));action(actions,"＋","Añadir instancia hija",()=>pickResource(i.id));}
    else action(actions,"↥","Importar definiciones TOML",()=> $("resource-files").click());
    action(actions,"↗","Ver padre / raíz",()=>{state.selected=p;renderAndSave();});
    if(!children(i).length)action(actions,"⌫","Eliminar instancia",()=>removeInstance(i.id),true);
    details.appendChild(actions);
  }
  function renderStructureRow(i, depthLevel, parent) {
    const kids=children(i), r=res(i), row=el("div","structure-node"+(state.selected===i.id?" selected":"")); row.style.marginLeft=Math.min(depthLevel*22,110)+"px"; row.tabIndex=0; row.setAttribute("role","button"); row.setAttribute("aria-pressed",String(state.selected===i.id));
    row.appendChild(el("span","node-glyph","◇"));const main=el("div","node-main");main.appendChild(el("div","node-title",title(i)));main.appendChild(el("div","node-meta",r?(r.info.tags.slice(0,3).join(" · ")||"Sin etiquetas"):"Importa el TOML para ver sus datos"));row.append(main,el("span","node-end",kids.length?kids.length+" hijos":"Instancia"));
    const select=()=>{state.selected=i.id;renderAndSave();};row.addEventListener("click",select);row.addEventListener("keydown",e=>{if(e.key==="Enter"||e.key===" "){e.preventDefault();select();}});parent.appendChild(row);kids.forEach(k=>renderStructureRow(k,depthLevel+1,parent));
  }
  function renderStructure() {
    structure.innerHTML="";const roots=state.workspace.content.instances;
    if(!roots.length){structure.innerHTML='<div class="empty-state"><div class="empty-orbit">⌘</div><h3>Empieza con una idea</h3><p>Crea una definición de recurso y una instancia. Después podrás anidar instancias para construir tu propia estructura.</p><div class="empty-actions"><button class="primary-button" id="empty-create">＋ Crear primer recurso</button><button class="secondary-button" id="empty-import">Importar archivos</button></div></div>';$("empty-create").addEventListener("click",openResourceDialog);$("empty-import").addEventListener("click",()=>$("resource-files").click());return;}
    const top=el("div","structure-topline");top.append(el("strong","","Árbol de instancias"),el("span","",flat().length+" elementos"));structure.appendChild(top);const list=el("div","structure-list");roots.forEach(i=>renderStructureRow(i,0,list));structure.appendChild(list);
  }
  function renderMap() {
    map.innerHTML="";const roots=state.workspace.content.instances;
    if(!roots.length){map.innerHTML='<div class="empty-state"><div class="empty-orbit">⌘</div><h3>El mapa está vacío</h3><p>Las instancias aparecerán aquí agrupadas por su jerarquía de contención.</p></div>';return;}
    const canvas=el("div","map-canvas");
    function add(i,parent){const col=el("div","map-column");const card=el("button","map-card"+(state.selected===i.id?" selected":""));card.append(el("strong","",title(i)),el("small","","Instancia · "+i.id.slice(0,8)));card.addEventListener("click",()=>{state.selected=i.id;renderAndSave();});col.appendChild(card);const kids=children(i);if(kids.length){const group=el("div","map-children");kids.forEach(k=>add(k,group));col.appendChild(group);}parent.appendChild(col);}
    roots.forEach(i=>add(i,canvas));map.appendChild(canvas);
  }
  function renderBreadcrumbs() {
    const b=$("breadcrumbs");b.innerHTML='<span class="crumb-muted">Workspace</span><span class="crumb-divider">/</span>';
    if(!state.selected){b.appendChild(el("span","","Raíz"));return;}
    let id=state.selected, chain=[];while(id){const i=find(id);if(!i)break;chain.unshift(i);id=parentId(id);}
    if(chain.length>1)b.appendChild(el("span","crumb-muted",title(chain[chain.length-2])));
    const current=el("button","",title(chain[chain.length-1]));current.style.cssText="border:0;background:transparent;color:#c4cde0;padding:0;font:inherit;cursor:pointer";current.addEventListener("click",()=>{state.selected=null;renderAndSave();});b.appendChild(current);
  }
  function render() {
    if(state.selected&&!find(state.selected))state.selected=null;
    renderTree();$("resource-count").textContent=Object.keys(state.resources).length;$("instance-count").textContent=flat().length;$("depth-count").textContent=state.selected?"Nivel "+(depth(state.selected)+1):"Raíz";renderBreadcrumbs();
    if(state.view==="map")renderMap();else renderStructure();structure.hidden=state.view!=="tree";map.hidden=state.view!=="map";
    $("tree-view-button").classList.toggle("active",state.view==="tree");$("map-view-button").classList.toggle("active",state.view==="map");$("tree-view-button").setAttribute("aria-pressed",String(state.view==="tree"));$("map-view-button").setAttribute("aria-pressed",String(state.view==="map"));
    renderDetails();$("add-instance").disabled=!Object.keys(state.resources).length;$("page-title").textContent=state.selected?title(find(state.selected)):"Espacio de trabajo";$("page-subtitle").textContent=state.selected?"Estás inspeccionando una instancia dentro de tu estructura.":"Organiza tus ideas en recursos y construye una estructura a tu manera.";
  }
  function renderAndSave(){save();render();}
  function openResourceDialog(){ $("resource-form").reset();$("create-instance-now").checked=true;dialog.showModal();$("resource-title").focus(); }
  function createResource(event) {
    event.preventDefault();const name=$("resource-title").value.trim();if(!name){notify("Escribe un nombre para el recurso.","error");return;}
    const id=uuid(),r={id,info:{title:name,description:$("resource-description").value.trim()||null,tags:$("resource-tags").value.split(",").map(t=>t.trim()).filter(Boolean)}};
    state.resources[id]=r;
    if($("create-instance-now").checked)addInstance(id,state.selected);
    dialog.close();renderAndSave();notify("Recurso creado en el navegador. Exporta el TOML para conservarlo.","success");
  }
  function addInstance(resourceId,parent) {
    const r=state.resources[resourceId];if(!r)throw new Error("No se encontró la definición del recurso.");
    const i={id:uuid(),source:{id:resourceId}};
    if(parent){const p=find(parent);if(!p)throw new Error("No se encontró la instancia padre.");if(!p.content)p.content={instances:[]};p.content.instances.push(i);if(!p.semantic)p.semantic={allocation:[]};p.semantic.allocation.push(i.id);state.expanded.add(parent);}
    else state.workspace.content.instances.push(i);
    state.selected=i.id;
  }
  function removeInstance(id) {
    const i=find(id);if(!i)return;if(children(i).length){notify("Solo puedes eliminar instancias hoja; elimina primero sus hijas.","error");return;}
    if(!confirm("¿Eliminar esta instancia del workspace? La definición del recurso se conservará."))return;
    const p=parentId(id),list=p?find(p).content.instances:state.workspace.content.instances,index=list.findIndex(x=>x.id===id);if(index>=0)list.splice(index,1);
    if(p){const par=find(p);if(par.semantic){par.semantic.allocation=par.semantic.allocation.filter(x=>x!==id);if(!par.semantic.allocation.length)delete par.semantic;}if(par.content&&!par.content.instances.length)delete par.content;}
    state.selected=p||null;renderAndSave();notify("Instancia eliminada; la definición se conserva.","success");
  }
  function pickResource(parent) {
    const ids=Object.keys(state.resources);if(!ids.length){openResourceDialog();return;}
    const choices=ids.map((id,n)=>(n+1)+". "+state.resources[id].info.title+" ["+id.slice(0,8)+"]").join("\n");
    const answer=prompt("¿Qué recurso quieres instanciar?\n\n"+choices+"\n\nEscribe el número:");
    if(answer===null)return;const n=Number(answer)-1;if(!Number.isInteger(n)||n<0||n>=ids.length){notify("Opción no válida.","error");return;}
    try{addInstance(ids[n],parent);renderAndSave();notify("Instancia añadida a la estructura.","success");}catch(e){notify(e.message,"error");}
  }
  function editResource(id) {
    const r=state.resources[id];if(!r)return;const name=prompt("Nombre del recurso",r.info.title);if(name===null)return;if(!name.trim()){notify("El nombre no puede estar vacío.","error");return;}
    const description=prompt("Descripción (vacía para quitarla)",r.info.description||"");if(description===null)return;
    const tags=prompt("Etiquetas separadas por comas",r.info.tags.join(", "));if(tags===null)return;
    r.info.title=name.trim();r.info.description=description.trim()||null;r.info.tags=tags.split(",").map(t=>t.trim()).filter(Boolean);renderAndSave();notify("Definición actualizada. Exporta el TOML para guardarla en tus archivos.","success");
  }
  async function importTomls(files) {
    let count=0,errors=[];
    for(const file of Array.from(files)){try{const r=normalizeResource(parseToml(await file.text()));state.resources[r.id]=r;count++;}catch(e){errors.push(file.name+": "+e.message);}}
    renderAndSave();if(count)notify(count+" definición(es) importada(s).","success");if(errors.length)notify(errors.slice(0,2).join(" · "),"error");
  }
  async function importWorkspace(file) {
    const w=normalizeWorkspace(JSON.parse(await file.text()));state.workspace=w;state.selected=null;state.expanded=new Set();renderAndSave();notify("Workspace importado. Importa también las definiciones TOML para ver sus títulos.","success");
  }
  function download(filename,content,mime) {
    const url=URL.createObjectURL(new Blob([content],{type:mime||"text/plain;charset=utf-8"}));const a=document.createElement("a");a.href=url;a.download=filename;document.body.appendChild(a);a.click();a.remove();setTimeout(()=>URL.revokeObjectURL(url),1000);
  }
  function exportWorkspace(){download("proyecto.json",JSON.stringify(normalizeWorkspace(state.workspace),null,2)+"\n","application/json;charset=utf-8");notify("Workspace exportado. Exporta también los TOML si modificaste definiciones.","success");}
  function exportResources() {
    const ids=Object.keys(state.resources);if(!ids.length){notify("No hay definiciones para exportar.","error");return;}
    ids.forEach(id=>{const r=state.resources[id];const lines=["id = "+quoteToml(r.id),"","[info]","title = "+quoteToml(r.info.title)];if(r.info.description)lines.push("description = "+quoteToml(r.info.description));lines.push("tags = ["+r.info.tags.map(quoteToml).join(", ")+"]","");download(id+".toml",lines.join("\n"),"text/plain;charset=utf-8");});
    notify("Se prepararon "+ids.length+" archivos TOML. El navegador puede pedir permiso para varias descargas.","success");
  }
  $("create-resource").addEventListener("click",openResourceDialog);$("cancel-resource").addEventListener("click",()=>dialog.close());$("resource-form").addEventListener("submit",createResource);
  $("add-instance").addEventListener("click",()=>Object.keys(state.resources).length?pickResource(state.selected):openResourceDialog());
  $("import-files").addEventListener("click",()=> $("resource-files").click());$("import-workspace").addEventListener("click",()=> $("workspace-file").click());$("export-workspace").addEventListener("click",exportWorkspace);$("export-resources").addEventListener("click",exportResources);
  $("resource-files").addEventListener("change",async e=>{if(e.target.files.length)await importTomls(e.target.files);e.target.value="";});
  $("workspace-file").addEventListener("change",async e=>{const f=e.target.files[0];if(f){try{await importWorkspace(f);}catch(err){notify("No se pudo importar el workspace: "+err.message,"error");}}e.target.value="";});
  $("import-files").addEventListener("dblclick",()=>$("workspace-file").click());$("import-files").title="Clic: importar TOML. Doble clic: importar workspace JSON.";
  $("search-input").addEventListener("input",e=>{state.search=e.target.value;renderTree();});
  $("collapse-all").addEventListener("click",()=>{state.expanded.clear();renderAndSave();});
  $("tree-view-button").addEventListener("click",()=>{state.view="tree";render();});$("map-view-button").addEventListener("click",()=>{state.view="map";render();});
  $("close-details").addEventListener("click",()=>{state.selected=null;renderAndSave();});$("help-button").addEventListener("click",()=>help.showModal());
  document.addEventListener("keydown",e=>{if(e.key==="/"&&!["INPUT","TEXTAREA"].includes(document.activeElement.tagName)&&!dialog.open&&!help.open){e.preventDefault();$("search-input").focus();}});
  load();render();
})();