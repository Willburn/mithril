(()=>{"use strict";var e,a,f,d,c,t={},b={};function r(e){var a=b[e];if(void 0!==a)return a.exports;var f=b[e]={id:e,loaded:!1,exports:{}};return t[e].call(f.exports,f,f.exports,r),f.loaded=!0,f.exports}r.m=t,r.c=b,e=[],r.O=(a,f,d,c)=>{if(!f){var t=1/0;for(i=0;i<e.length;i++){f=e[i][0],d=e[i][1],c=e[i][2];for(var b=!0,o=0;o<f.length;o++)(!1&c||t>=c)&&Object.keys(r.O).every((e=>r.O[e](f[o])))?f.splice(o--,1):(b=!1,c<t&&(t=c));if(b){e.splice(i--,1);var n=d();void 0!==n&&(a=n)}}return a}c=c||0;for(var i=e.length;i>0&&e[i-1][2]>c;i--)e[i]=e[i-1];e[i]=[f,d,c]},r.n=e=>{var a=e&&e.__esModule?()=>e.default:()=>e;return r.d(a,{a:a}),a},f=Object.getPrototypeOf?e=>Object.getPrototypeOf(e):e=>e.__proto__,r.t=function(e,d){if(1&d&&(e=this(e)),8&d)return e;if("object"==typeof e&&e){if(4&d&&e.__esModule)return e;if(16&d&&"function"==typeof e.then)return e}var c=Object.create(null);r.r(c);var t={};a=a||[null,f({}),f([]),f(f)];for(var b=2&d&&e;"object"==typeof b&&!~a.indexOf(b);b=f(b))Object.getOwnPropertyNames(b).forEach((a=>t[a]=()=>e[a]));return t.default=()=>e,r.d(c,t),c},r.d=(e,a)=>{for(var f in a)r.o(a,f)&&!r.o(e,f)&&Object.defineProperty(e,f,{enumerable:!0,get:a[f]})},r.f={},r.e=e=>Promise.all(Object.keys(r.f).reduce(((a,f)=>(r.f[f](e,a),a)),[])),r.u=e=>"assets/js/"+({42:"1bcabeba",53:"935f2afb",301:"0dadd2c9",335:"1386e018",477:"0ab7ecaa",613:"5efc9d3d",745:"a9095f31",1094:"e21cfb82",1331:"eb3cecd3",1971:"da6513d5",2307:"6759b17e",2512:"d07df43d",2535:"814f3328",2659:"346551de",2708:"5ee0e852",2852:"b1a5869c",3089:"a6aa9e1f",3148:"8bb94aa1",3511:"63969280",3608:"9e4087bc",3687:"eab6d850",3828:"c638a06a",3907:"855a8af1",3922:"3eb12003",3945:"1ea8ad02",4013:"01a85c17",4059:"3aecf4c2",4071:"4579318a",4163:"1d3fbc77",4195:"c4f5d8e4",4354:"298e1cbf",4528:"28c41bf0",4687:"646279b0",5038:"e7e087cc",5857:"d4f8d7b5",5968:"efe9c66f",6103:"ccc49370",6320:"3aa955b1",6430:"1dd8b324",6825:"b6aa46a7",6846:"319c539b",6880:"3a51032f",7338:"3488a21a",7395:"4d078e38",7485:"c7d749c3",7615:"2419ec42",7658:"c554d126",7695:"653f7965",7918:"17896441",8071:"73902fa9",8139:"012f7f96",8159:"bdc52102",8587:"bef1cd89",8610:"6875c492",8612:"f0ad3fbb",8834:"f9376de9",8985:"59730d2f",9387:"9494ffc1",9514:"1be78505",9518:"b48fcc4a",9697:"73fe69d8",9817:"14eb3368"}[e]||e)+"."+{42:"35b8a76a",53:"27aa879d",301:"95e4c9c1",335:"3648e735",477:"c7f29f10",613:"f1786fce",745:"97fcd20a",1094:"e708387e",1331:"284e01bd",1971:"627a1c50",2307:"8558e0ee",2512:"b9cbca08",2535:"cf614be9",2659:"10968b1c",2708:"e0ef8080",2852:"dfa1e08f",3089:"51fea4fd",3148:"b65a8482",3511:"12b795f0",3527:"b0c98fbc",3608:"ff562d32",3687:"84408522",3828:"e7e2a885",3907:"f5c1bb9b",3922:"2c1dea98",3945:"58336a8f",4013:"6004278c",4059:"50d82c20",4071:"aabd61a7",4163:"125bb106",4195:"4652450d",4354:"813e6148",4528:"bf55619a",4687:"b452327e",4972:"9aef2867",5038:"e404faa8",5857:"a2e83e1a",5968:"e9f73b17",6048:"8130eba0",6103:"ea41bce9",6320:"2d5c64e4",6430:"61a4fd9c",6825:"0ae00c28",6846:"c72e160b",6880:"91001878",7036:"d2024a0a",7338:"7701dcbd",7395:"e73e971c",7485:"725aff62",7615:"d99f1511",7658:"0578cf26",7695:"d2b0a695",7918:"87a8a0e3",8071:"dec6458b",8139:"fc0dac58",8159:"d004f5d1",8587:"85119220",8610:"8e7bd68e",8612:"1305652d",8834:"909a9420",8985:"e5d04753",9387:"1a5b5180",9514:"cb7a12ec",9518:"1d7243d9",9697:"4cab8638",9817:"8f382e71"}[e]+".js",r.miniCssF=e=>{},r.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),r.o=(e,a)=>Object.prototype.hasOwnProperty.call(e,a),d={},c="mithril-doc:",r.l=(e,a,f,t)=>{if(d[e])d[e].push(a);else{var b,o;if(void 0!==f)for(var n=document.getElementsByTagName("script"),i=0;i<n.length;i++){var l=n[i];if(l.getAttribute("src")==e||l.getAttribute("data-webpack")==c+f){b=l;break}}b||(o=!0,(b=document.createElement("script")).charset="utf-8",b.timeout=120,r.nc&&b.setAttribute("nonce",r.nc),b.setAttribute("data-webpack",c+f),b.src=e),d[e]=[a];var u=(a,f)=>{b.onerror=b.onload=null,clearTimeout(s);var c=d[e];if(delete d[e],b.parentNode&&b.parentNode.removeChild(b),c&&c.forEach((e=>e(f))),a)return a(f)},s=setTimeout(u.bind(null,void 0,{type:"timeout",target:b}),12e4);b.onerror=u.bind(null,b.onerror),b.onload=u.bind(null,b.onload),o&&document.head.appendChild(b)}},r.r=e=>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},r.nmd=e=>(e.paths=[],e.children||(e.children=[]),e),r.p="/doc/",r.gca=function(e){return e={17896441:"7918",63969280:"3511","1bcabeba":"42","935f2afb":"53","0dadd2c9":"301","1386e018":"335","0ab7ecaa":"477","5efc9d3d":"613",a9095f31:"745",e21cfb82:"1094",eb3cecd3:"1331",da6513d5:"1971","6759b17e":"2307",d07df43d:"2512","814f3328":"2535","346551de":"2659","5ee0e852":"2708",b1a5869c:"2852",a6aa9e1f:"3089","8bb94aa1":"3148","9e4087bc":"3608",eab6d850:"3687",c638a06a:"3828","855a8af1":"3907","3eb12003":"3922","1ea8ad02":"3945","01a85c17":"4013","3aecf4c2":"4059","4579318a":"4071","1d3fbc77":"4163",c4f5d8e4:"4195","298e1cbf":"4354","28c41bf0":"4528","646279b0":"4687",e7e087cc:"5038",d4f8d7b5:"5857",efe9c66f:"5968",ccc49370:"6103","3aa955b1":"6320","1dd8b324":"6430",b6aa46a7:"6825","319c539b":"6846","3a51032f":"6880","3488a21a":"7338","4d078e38":"7395",c7d749c3:"7485","2419ec42":"7615",c554d126:"7658","653f7965":"7695","73902fa9":"8071","012f7f96":"8139",bdc52102:"8159",bef1cd89:"8587","6875c492":"8610",f0ad3fbb:"8612",f9376de9:"8834","59730d2f":"8985","9494ffc1":"9387","1be78505":"9514",b48fcc4a:"9518","73fe69d8":"9697","14eb3368":"9817"}[e]||e,r.p+r.u(e)},(()=>{var e={1303:0,532:0};r.f.j=(a,f)=>{var d=r.o(e,a)?e[a]:void 0;if(0!==d)if(d)f.push(d[2]);else if(/^(1303|532)$/.test(a))e[a]=0;else{var c=new Promise(((f,c)=>d=e[a]=[f,c]));f.push(d[2]=c);var t=r.p+r.u(a),b=new Error;r.l(t,(f=>{if(r.o(e,a)&&(0!==(d=e[a])&&(e[a]=void 0),d)){var c=f&&("load"===f.type?"missing":f.type),t=f&&f.target&&f.target.src;b.message="Loading chunk "+a+" failed.\n("+c+": "+t+")",b.name="ChunkLoadError",b.type=c,b.request=t,d[1](b)}}),"chunk-"+a,a)}},r.O.j=a=>0===e[a];var a=(a,f)=>{var d,c,t=f[0],b=f[1],o=f[2],n=0;if(t.some((a=>0!==e[a]))){for(d in b)r.o(b,d)&&(r.m[d]=b[d]);if(o)var i=o(r)}for(a&&a(f);n<t.length;n++)c=t[n],r.o(e,c)&&e[c]&&e[c][0](),e[c]=0;return r.O(i)},f=self.webpackChunkmithril_doc=self.webpackChunkmithril_doc||[];f.forEach(a.bind(null,0)),f.push=a.bind(null,f.push.bind(f))})()})();