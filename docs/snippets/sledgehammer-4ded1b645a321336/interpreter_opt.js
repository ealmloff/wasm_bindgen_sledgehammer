let e,t,s,i,r,o,u,n,a,d,f,l,h,g,c,P,p,b,B,m,w,v,U;export function work_last_created(){c.Work()}export function last_needs_memory(){return!P.byteLength}export function update_last_memory(e){c.UpdateMemory(e)}function exOp(){switch(e&31){case 0:c.lastNode=c.lastNode.firstChild;break;case 1:c.lastNode=c.lastNode.nextSibling;break;case 2:c.lastNode=c.lastNode.parentNode;break;case 3:c.nodes[c.view.getUint32(c.u8BufPos,true)]=c.lastNode;c.u8BufPos+=4;break;case 4:c.lastNode=c.nodes[c.view.getUint32(c.u8BufPos,true)];c.u8BufPos+=4;break;case 5:return true;case 6:c.createFullElement();break;case 7:if(e&32){b=c.nodes[c.view.getUint32(c.u8BufPos,true)];c.u8BufPos+=4}else{b=c.lastNode}if(e&64){t=c.decodeU32();for(r=0;r<t;r++){b.appendChild(c.nodes[c.view.getUint32(c.u8BufPos,true)]);c.u8BufPos+=4}}else{b.appendChild(c.nodes[c.view.getUint32(c.u8BufPos,true)]);c.u8BufPos+=4}break;case 8:if(e&32){b=c.nodes[c.view.getUint32(c.u8BufPos,true)];c.u8BufPos+=4}else{b=c.lastNode}if(e&64){t=c.decodeU32();B=[];for(r=0;r<t;r++){B.push(c.nodes[c.view.getUint32(c.u8BufPos,true)]);c.u8BufPos+=4}b.replaceWith(...B)}else{b.replaceWith(c.nodes[c.view.getUint32(c.u8BufPos,true)]);c.u8BufPos+=4}break;case 9:if(e&32){b=c.nodes[c.view.getUint32(c.u8BufPos,true)];c.u8BufPos+=4}else{b=c.lastNode}if(e&64){t=c.decodeU32();B=[];for(r=0;r<t;r++){B.push(c.nodes[c.view.getUint32(c.u8BufPos,true)]);c.u8BufPos+=4}b.after(...B)}else{b.after(c.nodes[c.view.getUint32(c.u8BufPos,true)]);c.u8BufPos+=4}break;case 10:if(e&32){b=c.nodes[c.view.getUint32(c.u8BufPos,true)];c.u8BufPos+=4}else{b=c.lastNode}if(e&64){t=c.decodeU32();B=[];for(r=0;r<t;r++){B.push(c.nodes[c.view.getUint32(c.u8BufPos,true)]);c.u8BufPos+=4}b.before(...B)}else{b.before(c.nodes[c.view.getUint32(c.u8BufPos,true)]);c.u8BufPos+=4}break;case 11:if(e&32){c.nodes[c.view.getUint32(c.u8BufPos,true)].remove();c.u8BufPos+=4}else{c.lastNode.remove()}break;case 12:c.lastNode=document.createTextNode(c.strings.substring(c.strPos,c.strPos+=c.view.getUint16(c.u8BufPos,true)));c.u8BufPos+=2;if(e&32){c.nodes[c.view.getUint32(c.u8BufPos,true)]=c.lastNode;c.u8BufPos+=4}break;case 13:w=c.strings.substring(c.strPos,c.strPos+=c.view.getUint16(c.u8BufPos,true));c.u8BufPos+=2;if(e&32){c.lastNode=document.createElementNS(w,c.strings.substring(c.strPos,c.strPos+=c.view.getUint16(c.u8BufPos,true)));c.u8BufPos+=2}else{c.lastNode=document.createElement(w)}if(e&64){c.nodes[c.view.getUint32(c.u8BufPos,true)]=c.lastNode;c.u8BufPos+=4}break;case 14:if(e&32){v=c.view.getUint32(c.u8BufPos,true);c.u8BufPos+=4;c.nodes[v].textContent=c.strings.substring(c.strPos,c.strPos+=c.view.getUint16(c.u8BufPos,true));c.u8BufPos+=2}else{c.lastNode.textContent=c.strings.substring(c.strPos,c.strPos+=c.view.getUint16(c.u8BufPos,true));c.u8BufPos+=2}break;case 15:if(e&32){m=c.nodes[c.view.getUint32(c.u8BufPos,true)];c.u8BufPos+=4}else{m=c.lastNode}if(e&64){i=c.strings.substring(c.strPos,c.strPos+=c.view.getUint16(c.u8BufPos,true));c.u8BufPos+=2;m.setAttribute(i,c.strings.substring(c.strPos,c.strPos+=c.view.getUint16(c.u8BufPos,true)));c.u8BufPos+=2}else{m.setAttribute(N[c.view.getUint8(c.u8BufPos++)],c.strings.substring(c.strPos,c.strPos+=c.view.getUint16(c.u8BufPos,true)));c.u8BufPos+=2}break;case 16:if(e&32){m=c.nodes[c.view.getUint32(c.u8BufPos,true)];c.u8BufPos+=4}else{m=c.lastNode}i=c.strings.substring(c.strPos,c.strPos+=c.view.getUint16(c.u8BufPos,true));c.u8BufPos+=2;s=c.strings.substring(c.strPos,c.strPos+=c.view.getUint16(c.u8BufPos,true));c.u8BufPos+=2;o=c.strings.substring(c.strPos,c.strPos+=c.view.getUint16(c.u8BufPos,true));c.u8BufPos+=2;if(s==="style"){m.style[i]=o}else if(s!=null||s!=undefined){m.setAttributeNS(s,i,o)}break;case 17:if(e&32){m=c.nodes[c.view.getUint32(c.u8BufPos,true)];c.u8BufPos+=4}else{m=c.lastNode}if(e&64){m.removeAttribute(c.strings.substring(c.strPos,c.strPos+=c.view.getUint16(c.u8BufPos,true)));c.u8BufPos+=2}else{m.removeAttribute(N[c.view.getUint8(c.u8BufPos++)])}break;case 18:if(e&32){m=c.nodes[c.view.getUint32(c.u8BufPos,true)];c.u8BufPos+=4}else{m=c.lastNode}i=c.strings.substring(c.strPos,c.strPos+=c.view.getUint16(c.u8BufPos,true));c.u8BufPos+=2;m.removeAttributeNS(c.strings.substring(c.strPos,c.strPos+=c.view.getUint16(c.u8BufPos,true)),i);c.u8BufPos+=2;break;case 19:if(e&32){const u=c.view.getUint32(c.u8BufPos,true);c.lastNode=c.nodes[u].cloneNode(true);c.u8BufPos+=4}else{c.lastNode=c.lastNode.cloneNode(true)}if(e&64){c.nodes[c.view.getUint32(c.u8BufPos,true)]=c.lastNode;c.u8BufPos+=4}break;case 20:if(e&32){m=c.nodes[c.view.getUint32(c.u8BufPos,true)].cloneNode(true).firstChild;c.u8BufPos+=4}else{m=c.lastNode.cloneNode(true).firstChild}for(;m!==null;m=m.nextSibling){if(c.view.getUint8(c.u8BufPos++)===1){c.nodes[c.view.getUint32(c.u8BufPos,true)]=m;c.u8BufPos+=4}}break;default:break}}export class JsInterpreter{constructor(e,t,s,i,r){this.lastNode;this.nodes=[];this.parents=[];this.UpdateMemory(e);this.last_start_pos;this.metadata_ptr=t;this.ptr_ptr=s;this.str_ptr_ptr=i;this.str_len_ptr=r;this.strings="";this.strPos=0;this.decoder=new TextDecoder;this.idSize=1;c=this}NeedsMemory(){return this.view.buffer.byteLength===0}UpdateMemory(e){this.view=new DataView(e.buffer);P=e.buffer}Work(){p=this.view.getUint8(this.metadata_ptr);if(p&1){this.last_start_pos=this.view.getUint32(this.ptr_ptr,true)}this.u8BufPos=this.last_start_pos;if(p&2){t=this.view.getUint32(this.str_len_ptr,true);n=this.view.getUint32(this.str_ptr_ptr,true);if(t<100){if(p&4){this.strings=this.batchedAsciiDecode(n,t)}else{this.strings=this.utf8Decode(n,t)}}else{this.strings=this.decoder.decode(new DataView(this.view.buffer,n,t))}this.strPos=0}for(;;){e=this.view.getUint32(this.u8BufPos,true);this.u8BufPos+=4;if(exOp())return;e>>>=8;if(exOp())return;e>>>=8;if(exOp())return;e>>>=8;if(exOp())return}}createElement(){u=this.view.getUint8(this.u8BufPos++);if(u===255){u=document.createElement(this.strings.substring(this.strPos,this.strPos+=this.view.getUint16(this.u8BufPos,true)));this.u8BufPos+=2;return u}else{return document.createElement(k[u])}}createFullElement(){const e=this.decodeMaybeIdByteBool(),t=this.createElement();h=this.view.getUint8(this.u8BufPos++);for(r=0;r<h;r++){i=this.view.getUint8(this.u8BufPos++);switch(i){case 254:i=this.strings.substring(this.strPos,this.strPos+=this.view.getUint16(this.u8BufPos,true));this.u8BufPos+=2;s=this.strings.substring(this.strPos,this.strPos+=this.view.getUint16(this.u8BufPos,true));this.u8BufPos+=2;o=this.strings.substring(this.strPos,this.strPos+=this.view.getUint16(this.u8BufPos,true));this.u8BufPos+=2;t.setAttributeNS(s,i,o);break;case 255:i=this.strings.substring(this.strPos,this.strPos+=this.view.getUint16(this.u8BufPos,true));this.u8BufPos+=2;t.setAttribute(i,this.strings.substring(this.strPos,this.strPos+=this.view.getUint16(this.u8BufPos,true)));this.u8BufPos+=2;break;default:t.setAttribute(N[i],this.strings.substring(this.strPos,this.strPos+=this.view.getUint16(this.u8BufPos,true)));this.u8BufPos+=2;break}}const u=this.view.getUint8(this.u8BufPos++);for(let n=0;n<u;n++){t.appendChild(this.createFullElement())}if(e!==null){this.nodes[e]=t}return t}decodeMaybeIdByteBool(){if(this.view.getUint8(this.u8BufPos++)===0){return null}else{const e=this.view.getUint32(this.u8BufPos,true);this.u8BufPos+=4;return e}}decodeU32(){this.u8BufPos+=4;return this.view.getUint32(this.u8BufPos-4,true)}SetNode(e,t){this.nodes[e]=t}GetNode(e){return this.nodes[e]}utf8Decode(e,t){a=e;d=a+t;f="";while(a<d){l=this.view.getUint8(a++);if((l&128)===0){f+=String.fromCharCode(l)}else if((l&224)===192){f+=String.fromCharCode((l&31)<<6|this.view.getUint8(a++)&63)}else if((l&240)===224){f+=String.fromCharCode((l&31)<<12|(this.view.getUint8(a++)&63)<<6|this.view.getUint8(a++)&63)}else if((l&248)===240){let s=(l&7)<<18|(this.view.getUint8(a++)&63)<<12|(this.view.getUint8(a++)&63)<<6|this.view.getUint8(a++)&63;if(s>65535){s-=65536;f+=String.fromCharCode(s>>>10&1023|55296);s=56320|s&1023}f+=String.fromCharCode(s)}else{f+=String.fromCharCode(l)}}return f}batchedAsciiDecode(e,t){a=e;d=a+t;f="";g=a+(t/4|0)*4;while(a<g){l=this.view.getUint32(a);f+=String.fromCharCode(l>>24,(l&16711680)>>16,(l&65280)>>8,l&255);a+=4}while(a<d){f+=String.fromCharCode(this.view.getUint8(a++))}return f}}const k=["a","abbr","acronym","address","applet","area","article","aside","audio","b","base","bdi","bdo","bgsound","big","blink","blockquote","body","br","button","canvas","caption","center","cite","code","col","colgroup","content","data","datalist","dd","del","details","dfn","dialog","dir","div","dl","dt","em","embed","fieldset","figcaption","figure","font","footer","form","frame","frameset","h1","head","header","hgroup","hr","html","i","iframe","image","img","input","ins","kbd","keygen","label","legend","li","link","main","map","mark","marquee","menu","menuitem","meta","meter","nav","nobr","noembed","noframes","noscript","object","ol","optgroup","option","output","p","param","picture","plaintext","portal","pre","progress","q","rb","rp","rt","rtc","ruby","s","samp","script","section","select","shadow","slot","small","source","spacer","span","strike","strong","style","sub","summary","sup","table","tbody","td","template","textarea","tfoot","th","thead","time","title","tr","track","tt","u","ul","var","video","wbr","xmp"];const N=["accept-charset","accept","accesskey","action","align","allow","alt","aria-atomic","aria-busy","aria-controls","aria-current","aria-describedby","aria-description","aria-details","aria-disabled","aria-dropeffect","aria-errormessage","aria-flowto","aria-grabbed","aria-haspopup","aria-hidden","aria-invalid","aria-keyshortcuts","aria-label","aria-labelledby","aria-live","aria-owns","aria-relevant","aria-roledescription","async","autocapitalize","autocomplete","autofocus","autoplay","background","bgcolor","border","buffered","capture","challenge","charset","checked","cite","class","code","codebase","color","cols","colspan","content","contenteditable","contextmenu","controls","coords","crossorigin","csp","data","datetime","decoding","default","defer","dir","dirname","disabled","download","draggable","enctype","enterkeyhint","for","form","formaction","formenctype","formmethod","formnovalidate","formtarget","headers","height","hidden","high","href","hreflang","http-equiv","icon","id","importance","inputmode","integrity","intrinsicsize","ismap","itemprop","keytype","kind","label","lang","language","list","loading","loop","low","manifest","max","maxlength","media","method","min","minlength","multiple","muted","name","novalidate","open","optimum","pattern","ping","placeholder","poster","preload","radiogroup","readonly","referrerpolicy","rel","required","reversed","role","rows","rowspan","sandbox","scope","scoped","selected","shape","size","sizes","slot","span","spellcheck","src","srcdoc","srclang","srcset","start","step","style","summary","tabindex","target","title","translate","type","usemap","value","width","wrap"];