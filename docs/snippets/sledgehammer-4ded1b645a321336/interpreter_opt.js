let t,e,s,i,r,o,u,a,n,d,l,f,h,g,P,c,b,p,B,m,w,v,U,k;export function work_last_created(){c.Work()}export function last_needs_memory(){return!b.byteLength}export function update_last_memory(t){c.UpdateMemory(t)}function exOp(){switch(t&31){case 0:c.lastNode=c.lastNode.firstChild;break;case 1:c.lastNode=c.lastNode.nextSibling;break;case 2:c.lastNode=c.lastNode.parentNode;break;case 3:c.nodes[c.view.getUint32(c.u8BufPos,true)]=c.lastNode;c.u8BufPos+=4;break;case 4:c.lastNode=c.nodes[c.view.getUint32(c.u8BufPos,true)];c.u8BufPos+=4;break;case 5:return true;case 6:c.lastNode=c.createFullElement();break;case 7:if(t&32){B=c.nodes[c.view.getUint32(c.u8BufPos,true)];c.u8BufPos+=4}else{B=c.lastNode}if(t&64){e=c.decodeU32();for(r=0;r<e;r++){B.appendChild(c.nodes[c.view.getUint32(c.u8BufPos,true)]);c.u8BufPos+=4}}else{B.appendChild(c.nodes[c.view.getUint32(c.u8BufPos,true)]);c.u8BufPos+=4}break;case 8:if(t&32){B=c.nodes[c.view.getUint32(c.u8BufPos,true)];c.u8BufPos+=4}else{B=c.lastNode}if(t&64){e=c.decodeU32();m=[];for(r=0;r<e;r++){m.push(c.nodes[c.view.getUint32(c.u8BufPos,true)]);c.u8BufPos+=4}B.replaceWith(...m)}else{B.replaceWith(c.nodes[c.view.getUint32(c.u8BufPos,true)]);c.u8BufPos+=4}break;case 9:if(t&32){B=c.nodes[c.view.getUint32(c.u8BufPos,true)];c.u8BufPos+=4}else{B=c.lastNode}if(t&64){e=c.decodeU32();m=[];for(r=0;r<e;r++){m.push(c.nodes[c.view.getUint32(c.u8BufPos,true)]);c.u8BufPos+=4}B.after(...m)}else{B.after(c.nodes[c.view.getUint32(c.u8BufPos,true)]);c.u8BufPos+=4}break;case 10:if(t&32){B=c.nodes[c.view.getUint32(c.u8BufPos,true)];c.u8BufPos+=4}else{B=c.lastNode}if(t&64){e=c.decodeU32();m=[];for(r=0;r<e;r++){m.push(c.nodes[c.view.getUint32(c.u8BufPos,true)]);c.u8BufPos+=4}B.before(...m)}else{B.before(c.nodes[c.view.getUint32(c.u8BufPos,true)]);c.u8BufPos+=4}break;case 11:if(t&32){c.nodes[c.view.getUint32(c.u8BufPos,true)].remove();c.u8BufPos+=4}else{c.lastNode.remove()}break;case 12:c.lastNode=document.createTextNode(c.strings.substring(c.strPos,c.strPos+=c.view.getUint16(c.u8BufPos,true)));c.u8BufPos+=2;if(t&32){c.nodes[c.view.getUint32(c.u8BufPos,true)]=c.lastNode;c.u8BufPos+=4}break;case 13:c.lastNode=c.createElement();if(t&32){c.nodes[c.view.getUint32(c.u8BufPos,true)]=c.lastNode;c.u8BufPos+=4}break;case 14:if(t&32){U=c.view.getUint32(c.u8BufPos,true);c.u8BufPos+=4;c.nodes[U].textContent=c.strings.substring(c.strPos,c.strPos+=c.view.getUint16(c.u8BufPos,true));c.u8BufPos+=2}else{c.lastNode.textContent=c.strings.substring(c.strPos,c.strPos+=c.view.getUint16(c.u8BufPos,true));c.u8BufPos+=2}break;case 15:if(t&32){w=c.nodes[c.view.getUint32(c.u8BufPos,true)];c.u8BufPos+=4}else{w=c.lastNode}if(t&64){r=c.view.getUint32(c.u8BufPos,true);c.u8BufPos+=4;i=c.strings.substring(c.strPos,c.strPos+=r&65535);if(t&128){w.setAttributeNS(c.strings.substring(c.strPos,c.strPos+=(r&4294901760)>>>16),i,c.strings.substring(c.strPos,c.strPos+=c.view.getUint16(c.u8BufPos,true)));c.u8BufPos+=2}else{w.setAttribute(i,c.strings.substring(c.strPos,c.strPos+=(r&4294901760)>>>16))}}else{r=c.view.getUint32(c.u8BufPos,true);c.u8BufPos+=3;if(t&128){s=c.strings.substring(c.strPos,c.strPos+=r&65535);w.setAttributeNS(s,y[(r&16711680)>>>16],c.strings.substring(c.strPos,c.strPos+=c.view.getUint16(c.u8BufPos,true)));c.u8BufPos+=2}else{w.setAttribute(y[r&255],c.strings.substring(c.strPos,c.strPos+=(r&16776960)>>>8))}}break;case 16:if(t&32){w=c.nodes[c.view.getUint32(c.u8BufPos,true)];c.u8BufPos+=4}else{w=c.lastNode}if(t&64){if(t&128){r=c.view.getUint32(c.u8BufPos,true);c.u8BufPos+=4;i=c.strings.substring(c.strPos,c.strPos+=r&65535);w.removeAttributeNS(c.strings.substring(c.strPos,c.strPos+=(r&4294901760)>>>16),i)}else{w.removeAttribute(c.strings.substring(c.strPos,c.strPos+=c.view.getUint16(c.u8BufPos,true)));c.u8BufPos+=2}}else{if(t&128){r=c.view.getUint32(c.u8BufPos,true);c.u8BufPos+=3;i=y[r&255];w.removeAttributeNS(c.strings.substring(c.strPos,c.strPos+=(r&16776960)>>>8),i)}else{w.removeAttribute(y[c.view.getUint8(c.u8BufPos++)])}}break;case 17:if(t&32){w=c.nodes[c.view.getUint32(c.u8BufPos,true)];c.u8BufPos+=4}else{w=c.lastNode}r=c.view.getUint32(c.u8BufPos,true);c.u8BufPos+=4;w.style.setProperty(c.strings.substring(c.strPos,c.strPos+=r&65535),c.strings.substring(c.strPos,c.strPos+=(r&4294901760)>>>16));break;case 18:if(t&32){w=c.nodes[c.view.getUint32(c.u8BufPos,true)];c.u8BufPos+=4}else{w=c.lastNode}w.style.removeProperty(c.strings.substring(c.strPos,c.strPos+=c.view.getUint16(c.u8BufPos,true)));c.u8BufPos+=2;break;case 19:if(t&32){c.lastNode=c.nodes[c.view.getUint32(c.u8BufPos,true)].cloneNode(true);c.u8BufPos+=4}else{c.lastNode=c.lastNode.cloneNode(true)}if(t&64){c.nodes[c.view.getUint32(c.u8BufPos,true)]=c.lastNode;c.u8BufPos+=4}break;case 20:if(t&32){w=c.nodes[c.view.getUint32(c.u8BufPos,true)].cloneNode(true).firstChild;c.u8BufPos+=4}else{w=c.lastNode.cloneNode(true).firstChild}for(;w!==null;w=w.nextSibling){if(c.view.getUint8(c.u8BufPos++)===1){c.nodes[c.view.getUint32(c.u8BufPos,true)]=w;c.u8BufPos+=4}}break;default:break}}export class JsInterpreter{constructor(t,e,s,i,r){this.lastNode;this.nodes=[];this.parents=[];this.UpdateMemory(t);this.last_start_pos;this.last_str_start;this.metadata_ptr=e;this.ptr_ptr=s;this.str_ptr_ptr=i;this.str_len_ptr=r;this.strings="";this.strPos=0;this.decoder=new TextDecoder;this.idSize=1;c=this}NeedsMemory(){return this.view.buffer.byteLength===0}UpdateMemory(t){this.view=new DataView(t.buffer);b=t.buffer}Work(){p=this.view.getUint8(this.metadata_ptr);if(p&1){this.last_start_pos=this.view.getUint32(this.ptr_ptr,true)}this.u8BufPos=this.last_start_pos;if(p&4){e=this.view.getUint32(this.str_len_ptr,true);if(p&2){this.last_str_start=this.view.getUint32(this.str_ptr_ptr,true)}if(p&8){d=this.last_str_start;this.strings="";P=d+(e/4|0)*4;while(d<P){h=this.view.getUint32(d);this.strings+=String.fromCharCode(h>>24,(h&16711680)>>16,(h&65280)>>8,h&255);d+=4}switch(this.last_str_start+e-d){case 3:h=this.view.getUint32(d);this.strings+=String.fromCharCode(h>>24,(h&16711680)>>16,(h&65280)>>8);break;case 2:h=this.view.getUint16(d);this.strings+=String.fromCharCode(h>>8,h&255);break;case 1:this.strings+=String.fromCharCode(this.view.getUint8(d));break;case 0:break}}else{this.strings=this.decoder.decode(new DataView(this.view.buffer,this.last_str_start,e))}this.strPos=0}for(;;){t=this.view.getUint32(this.u8BufPos,true);this.u8BufPos+=4;if(exOp())return;t>>>=8;if(exOp())return;t>>>=8;if(exOp())return;t>>>=8;if(exOp())return}}createElement(){o=this.view.getUint32(this.u8BufPos,true);a=o&255;switch(a){case 255:this.u8BufPos+=4;a=document.createElement(N[(o&65280)>>>8],this.strings.substring(this.strPos,this.strPos+=(o&4294901760)>>>16));return a;case 254:this.u8BufPos+=3;a=document.createElement(this.strings.substring(this.strPos,this.strPos+=(o&16776960)>>>8));return a;case 253:this.u8BufPos+=3;a=this.strings.substring(this.strPos,this.strPos+=(o&16776960)>>>8);a=document.createElementNS(this.strings.substring(this.strPos,this.strPos+=this.view.getUint16(this.u8BufPos,true)),a);this.u8BufPos+=2;return a;default:this.u8BufPos++;return document.createElement(N[a])}}createFullElement(){const t=this.decodeMaybeIdByteBool(),e=this.createElement();o=this.view.getUint16(this.u8BufPos,true);this.u8BufPos+=2;g=o&255;const a=(o&65280)>>>8;for(r=0;r<g;r++){o=this.view.getUint32(this.u8BufPos,true);i=o&255;switch(i){case 255:this.u8BufPos+=4;i=y[this.view.getUint8((o&65280)>>>8)];e.setAttributeNS(this.strings.substring(this.strPos,this.strPos+=(o&4294901760)>>>16),i);break;case 254:this.u8BufPos++;o=this.view.getUint32(this.u8BufPos,true);this.u8BufPos+=4;i=this.strings.substring(this.strPos,this.strPos+=o&65535);e.setAttribute(i,this.strings.substring(this.strPos,this.strPos+=(o&4294901760)>>>16));break;case 253:this.u8BufPos+=3;i=this.strings.substring(this.strPos,this.strPos+=(o&16776960)>>>8);o=this.view.getUint32(this.u8BufPos,true);this.u8BufPos+=4;s=this.strings.substring(this.strPos,this.strPos+=o&65535);u=this.strings.substring(this.strPos,this.strPos+=(o&4294901760)>>>16);e.setAttributeNS(s,i,u);break;default:this.u8BufPos+=3;e.setAttribute(y[i],this.strings.substring(this.strPos,this.strPos+=(o&16776960)>>>8));break}}for(let n=0;n<a;n++){e.appendChild(this.createFullElement())}if(t!==null){this.nodes[t]=e}return e}decodeMaybeIdByteBool(){if(this.view.getUint8(this.u8BufPos++)===0){return null}else{const t=this.view.getUint32(this.u8BufPos,true);this.u8BufPos+=4;return t}}decodeU32(){this.u8BufPos+=4;return this.view.getUint32(this.u8BufPos-4,true)}SetNode(t,e){this.nodes[t]=e}GetNode(t){return this.nodes[t]}}const N=["a","abbr","acronym","address","applet","area","article","aside","audio","b","base","bdi","bdo","bgsound","big","blink","blockquote","body","br","button","canvas","caption","center","cite","code","col","colgroup","content","data","datalist","dd","del","details","dfn","dialog","dir","div","dl","dt","em","embed","fieldset","figcaption","figure","font","footer","form","frame","frameset","h1","head","header","hgroup","hr","html","i","iframe","image","img","input","ins","kbd","keygen","label","legend","li","link","main","map","mark","marquee","menu","menuitem","meta","meter","nav","nobr","noembed","noframes","noscript","object","ol","optgroup","option","output","p","param","picture","plaintext","portal","pre","progress","q","rb","rp","rt","rtc","ruby","s","samp","script","section","select","shadow","slot","small","source","spacer","span","strike","strong","style","sub","summary","sup","table","tbody","td","template","textarea","tfoot","th","thead","time","title","tr","track","tt","u","ul","var","video","wbr","xmp"];const y=["accept-charset","accept","accesskey","action","align","allow","alt","aria-atomic","aria-busy","aria-controls","aria-current","aria-describedby","aria-description","aria-details","aria-disabled","aria-dropeffect","aria-errormessage","aria-flowto","aria-grabbed","aria-haspopup","aria-hidden","aria-invalid","aria-keyshortcuts","aria-label","aria-labelledby","aria-live","aria-owns","aria-relevant","aria-roledescription","async","autocapitalize","autocomplete","autofocus","autoplay","background","bgcolor","border","buffered","capture","challenge","charset","checked","cite","class","code","codebase","color","cols","colspan","content","contenteditable","contextmenu","controls","coords","crossorigin","csp","data","datetime","decoding","default","defer","dir","dirname","disabled","download","draggable","enctype","enterkeyhint","for","form","formaction","formenctype","formmethod","formnovalidate","formtarget","headers","height","hidden","high","href","hreflang","http-equiv","icon","id","importance","inputmode","integrity","intrinsicsize","ismap","itemprop","keytype","kind","label","lang","language","list","loading","loop","low","manifest","max","maxlength","media","method","min","minlength","multiple","muted","name","novalidate","open","optimum","pattern","ping","placeholder","poster","preload","radiogroup","readonly","referrerpolicy","rel","required","reversed","role","rows","rowspan","sandbox","scope","scoped","selected","shape","size","sizes","slot","span","spellcheck","src","srcdoc","srclang","srcset","start","step","style","summary","tabindex","target","title","translate","type","usemap","value","width","wrap"];