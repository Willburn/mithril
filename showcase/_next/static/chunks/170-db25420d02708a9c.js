(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[170],{861:function(a,b,c){"use strict";c.d(b,{FT:function(){return g}});var d=c(7294),e=c(5893);let f=["as","disabled"];function g({tagName:a,disabled:b,href:c,target:d,rel:e,role:f,onClick:g,tabIndex:h=0,type:i}){a||(a=null!=c||null!=d||null!=e?"a":"button");let j={tagName:a};if("button"===a)return[{type:i||"button",disabled:b},j];let k=d=>{var e;if((b||"a"===a&&(!(e=c)||"#"===e.trim()))&&d.preventDefault(),b){d.stopPropagation();return}null==g||g(d)},l=a=>{" "===a.key&&(a.preventDefault(),k(a))};return"a"===a&&(c||(c="#"),b&&(c=void 0)),[{role:null!=f?f:"button",disabled:void 0,tabIndex:b?void 0:h,href:c,target:"a"===a?d:void 0,"aria-disabled":b||void 0,rel:"a"===a?e:void 0,onClick:k,onKeyDown:l},j]}let h=d.forwardRef((a,b)=>{let{as:c,disabled:d}=a,h=function(a,b){if(null==a)return{};var c,d,e={},f=Object.keys(a);for(d=0;d<f.length;d++)c=f[d],b.indexOf(c)>=0||(e[c]=a[c]);return e}(a,f),[i,{tagName:j}]=g(Object.assign({tagName:c,disabled:d},h));return(0,e.jsx)(j,Object.assign({},h,i,{ref:b}))});h.displayName="Button",b.ZP=h},9361:function(a,b){"use strict";b.Z=function(a,b,c){return b in a?Object.defineProperty(a,b,{value:c,enumerable:!0,configurable:!0,writable:!0}):a[b]=c,a}},4184:function(a,b){var c,d; /*!
  Copyright (c) 2018 Jed Watson.
  Licensed under the MIT License (MIT), see
  http://jedwatson.github.io/classnames
*/ !function(){"use strict";var e={}.hasOwnProperty;function f(){for(var a=[],b=0;b<arguments.length;b++){var c=arguments[b];if(c){var d=typeof c;if("string"===d||"number"===d)a.push(c);else if(Array.isArray(c)){if(c.length){var g=f.apply(null,c);g&&a.push(g)}}else if("object"===d){if(c.toString===Object.prototype.toString)for(var h in c)e.call(c,h)&&c[h]&&a.push(h);else a.push(c.toString())}}}return a.join(" ")}a.exports?(f.default=f,a.exports=f):void 0!==(d=(function(){return f}).apply(b,c=[]))&&(a.exports=d)}()},1143:function(a){"use strict";var b=function(a,b,c,d,e,f,g,h){if(!a){var i;if(void 0===b)i=Error("Minified exception occurred; use the non-minified dev environment for the full error message and additional helpful warnings.");else{var j=[c,d,e,f,g,h],k=0;(i=Error(b.replace(/%s/g,function(){return j[k++]}))).name="Invariant Violation"}throw i.framesToPop=1,i}};a.exports=b},8045:function(a,b,c){"use strict";Object.defineProperty(b,"__esModule",{value:!0});var d,e=c(9361).Z,f=c(4941).Z,g=c(3929).Z;Object.defineProperty(b,"__esModule",{value:!0}),b.default=function(a){var b,c,d=a.src,m=a.sizes,o=a.unoptimized,q=void 0!==o&&o,v=a.priority,B=void 0!==v&&v,D=a.loading,E=a.lazyRoot,F=void 0===E?null:E,G=a.lazyBoundary,H=a.className,I=a.quality,J=a.width,K=a.height,L=a.style,M=a.objectFit,N=a.objectPosition,O=a.onLoadingComplete,P=a.placeholder,Q=void 0===P?"empty":P,R=a.blurDataURL,S=p(a,["src","sizes","unoptimized","priority","loading","lazyRoot","lazyBoundary","className","quality","width","height","style","objectFit","objectPosition","onLoadingComplete","placeholder","blurDataURL"]),T=h.useContext(l.ImageConfigContext),U=h.useMemo(function(){var a=s||T||j.imageConfigDefault,b=g(a.deviceSizes).concat(g(a.imageSizes)).sort(function(a,b){return a-b}),c=a.deviceSizes.sort(function(a,b){return a-b});return n({},a,{allSizes:b,deviceSizes:c})},[T]),V=S,W=m?"responsive":"intrinsic";"layout"in V&&(V.layout&&(W=V.layout),delete V.layout);var X=A;if("loader"in V){if(V.loader){var Y,Z=V.loader;X=function(a){a.config;var b=p(a,["config"]);return Z(b)}}delete V.loader}var $="";if(x(d)){var _=w(d)?d.default:d;if(!_.src)throw Error("An object should only be passed to the image component src parameter if it comes from a static image import. It must include src. Received ".concat(JSON.stringify(_)));if(R=R||_.blurDataURL,$=_.src,(!W||"fill"!==W)&&(K=K||_.height,J=J||_.width,!_.height||!_.width))throw Error("An object should only be passed to the image component src parameter if it comes from a static image import. It must include height and width. Received ".concat(JSON.stringify(_)))}d="string"==typeof d?d:$;var aa=!B&&("lazy"===D|| void 0===D);(d.startsWith("data:")||d.startsWith("blob:"))&&(q=!0,aa=!1),t.has(d)&&(aa=!1),r&&(q=!0);var ab=f(h.useState(!1),2),ac=ab[0],ad=ab[1],ae=f(k.useIntersection({rootRef:F,rootMargin:G||"200px",disabled:!aa}),3),af=ae[0],ag=ae[1],ah=ae[2],ai=!aa||ag,aj={boxSizing:"border-box",display:"block",overflow:"hidden",width:"initial",height:"initial",background:"none",opacity:1,border:0,margin:0,padding:0},ak={boxSizing:"border-box",display:"block",width:"initial",height:"initial",background:"none",opacity:1,border:0,margin:0,padding:0},al=!1,am=z(J),an=z(K),ao=z(I),ap=Object.assign({},L,{position:"absolute",top:0,left:0,bottom:0,right:0,boxSizing:"border-box",padding:0,border:"none",margin:"auto",display:"block",width:0,height:0,minWidth:"100%",maxWidth:"100%",minHeight:"100%",maxHeight:"100%",objectFit:M,objectPosition:N}),aq="blur"!==Q||ac?{}:{backgroundSize:M||"cover",backgroundPosition:N||"0% 0%",filter:"blur(20px)",backgroundImage:'url("'.concat(R,'")')};if("fill"===W)aj.display="block",aj.position="absolute",aj.top=0,aj.left=0,aj.bottom=0,aj.right=0;else if(void 0!==am&& void 0!==an){var ar=an/am,as=isNaN(ar)?"100%":"".concat(100*ar,"%");"responsive"===W?(aj.display="block",aj.position="relative",al=!0,ak.paddingTop=as):"intrinsic"===W?(aj.display="inline-block",aj.position="relative",aj.maxWidth="100%",al=!0,ak.maxWidth="100%",b="data:image/svg+xml,%3csvg%20xmlns=%27http://www.w3.org/2000/svg%27%20version=%271.1%27%20width=%27".concat(am,"%27%20height=%27").concat(an,"%27/%3e")):"fixed"===W&&(aj.display="inline-block",aj.position="relative",aj.width=am,aj.height=an)}var at={src:u,srcSet:void 0,sizes:void 0};ai&&(at=y({config:U,src:d,unoptimized:q,layout:W,width:am,quality:ao,sizes:m,loader:X}));var au=d,av="imagesrcset",aw="imagesizes";aw="imageSizes";var ax=(e(c={},"imageSrcSet",at.srcSet),e(c,aw,at.sizes),c),ay=h.default.useLayoutEffect,az=h.useRef(O),aA=h.useRef(d);h.useEffect(function(){az.current=O},[O]),ay(function(){aA.current!==d&&(ah(),aA.current=d)},[ah,d]);var aB=n({isLazy:aa,imgAttributes:at,heightInt:an,widthInt:am,qualityInt:ao,layout:W,className:H,imgStyle:ap,blurStyle:aq,loading:D,config:U,unoptimized:q,placeholder:Q,loader:X,srcString:au,onLoadingCompleteRef:az,setBlurComplete:ad,setIntersection:af,isVisible:ai,noscriptSizes:m},V);return h.default.createElement(h.default.Fragment,null,h.default.createElement("span",{style:aj},al?h.default.createElement("span",{style:ak},b?h.default.createElement("img",{style:{display:"block",maxWidth:"100%",width:"initial",height:"initial",background:"none",opacity:1,border:0,margin:0,padding:0},alt:"","aria-hidden":!0,src:b}):null):null,h.default.createElement(C,Object.assign({},aB))),B?h.default.createElement(i.default,null,h.default.createElement("link",Object.assign({key:"__nimg-"+at.src+at.srcSet+at.sizes,rel:"preload",as:"image",href:at.srcSet?void 0:at.src},ax))):null)};var h=function(a){if(a&&a.__esModule)return a;if(null===a||"object"!=typeof a&&"function"!=typeof a)return{default:a};var b=o();if(b&&b.has(a))return b.get(a);var c={},d=Object.defineProperty&&Object.getOwnPropertyDescriptor;for(var e in a)if(Object.prototype.hasOwnProperty.call(a,e)){var f=d?Object.getOwnPropertyDescriptor(a,e):null;f&&(f.get||f.set)?Object.defineProperty(c,e,f):c[e]=a[e]}return c.default=a,b&&b.set(a,c),c}(c(7294)),i=(d=c(5443),d&&d.__esModule?d:{default:d}),j=c(9309),k=c(7190),l=c(9977);c(3794);var m=c(2392);function n(){return(n=Object.assign||function(a){for(var b=1;b<arguments.length;b++){var c=arguments[b];for(var d in c)Object.prototype.hasOwnProperty.call(c,d)&&(a[d]=c[d])}return a}).apply(this,arguments)}function o(){if("function"!=typeof WeakMap)return null;var a=new WeakMap;return o=function(){return a},a}function p(a,b){if(null==a)return{};var c,d,e={},f=Object.keys(a);for(d=0;d<f.length;d++)c=f[d],b.indexOf(c)>=0||(e[c]=a[c]);return e}var q={deviceSizes:[640,750,828,1080,1200,1920,2048,3840],imageSizes:[16,32,48,64,96,128,256,384],path:"/_next/image",loader:"default",experimentalUnoptimized:!0},r=(q.experimentalRemotePatterns,q.experimentalUnoptimized),s={deviceSizes:[640,750,828,1080,1200,1920,2048,3840],imageSizes:[16,32,48,64,96,128,256,384],path:"/_next/image",loader:"default",experimentalUnoptimized:!0},t=new Set,u="data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7",v=new Map([["default",function(a){var b=a.config,c=a.src,d=a.width,e=a.quality;return c.endsWith(".svg")&&!b.dangerouslyAllowSVG?c:"".concat(m.normalizePathTrailingSlash(b.path),"?url=").concat(encodeURIComponent(c),"&w=").concat(d,"&q=").concat(e||75)}],["imgix",function(a){var b=a.config,c=a.src,d=a.width,e=a.quality,f=new URL("".concat(b.path).concat(D(c))),g=f.searchParams;return g.set("auto",g.get("auto")||"format"),g.set("fit",g.get("fit")||"max"),g.set("w",g.get("w")||d.toString()),e&&g.set("q",e.toString()),f.href}],["cloudinary",function(a){var b=a.config,c=a.src,d=a.width,e=a.quality,f=["f_auto","c_limit","w_"+d,"q_"+(e||"auto")].join(",")+"/";return"".concat(b.path).concat(f).concat(D(c))}],["akamai",function(a){var b=a.config,c=a.src,d=a.width;return"".concat(b.path).concat(D(c),"?imwidth=").concat(d)}],["custom",function(a){var b=a.src;throw Error('Image with src "'.concat(b,'" is missing "loader" prop.')+"\nRead more: https://nextjs.org/docs/messages/next-image-missing-loader")}],]);function w(a){return void 0!==a.default}function x(a){var b;return"object"==typeof a&&(w(a)|| void 0!==(b=a).src)}function y(a){var b=a.config,c=a.src,d=a.unoptimized,e=a.layout,f=a.width,h=a.quality,i=a.sizes,j=a.loader;if(d)return{src:c,srcSet:void 0,sizes:void 0};var k=function(a,b,c,d){var e=a.deviceSizes,f=a.allSizes;if(d&&("fill"===c||"responsive"===c)){for(var h=/(^|\s)(1?\d?\d)vw/g,i=[];j=h.exec(d);j)i.push(parseInt(j[2]));if(i.length){var j,k,l=.01*(k=Math).min.apply(k,g(i));return{widths:f.filter(function(a){return a>=e[0]*l}),kind:"w"}}return{widths:f,kind:"w"}}return"number"!=typeof b||"fill"===c||"responsive"===c?{widths:e,kind:"w"}:{widths:g(new Set([b,2*b].map(function(a){return f.find(function(b){return b>=a})||f[f.length-1]}))),kind:"x"}}(b,f,e,i),l=k.widths,m=k.kind,n=l.length-1;return{sizes:i||"w"!==m?i:"100vw",srcSet:l.map(function(a,d){return"".concat(j({config:b,src:c,quality:h,width:a})," ").concat("w"===m?a:d+1).concat(m)}).join(", "),src:j({config:b,src:c,quality:h,width:l[n]})}}function z(a){return"number"==typeof a?a:"string"==typeof a?parseInt(a,10):void 0}function A(a){var b,c=(null==(b=a.config)?void 0:b.loader)||"default",d=v.get(c);if(d)return d(a);throw Error('Unknown "loader" found in "next.config.js". Expected: '.concat(j.VALID_LOADERS.join(", "),". Received: ").concat(c))}function B(a,b,c,d,e,f){a&&a.src!==u&&a["data-loaded-src"]!==b&&(a["data-loaded-src"]=b,("decode"in a?a.decode():Promise.resolve()).catch(function(){}).then(function(){if(a.parentNode&&(t.add(b),"blur"===d&&f(!0),null==e?void 0:e.current)){var c=a.naturalWidth,g=a.naturalHeight;e.current({naturalWidth:c,naturalHeight:g})}}))}var C=function(a){var b=a.imgAttributes,c=(a.heightInt,a.widthInt),d=a.qualityInt,e=a.layout,f=a.className,g=a.imgStyle,i=a.blurStyle,j=a.isLazy,k=a.placeholder,l=a.loading,m=a.srcString,o=a.config,q=a.unoptimized,r=a.loader,s=a.onLoadingCompleteRef,t=a.setBlurComplete,u=a.setIntersection,v=a.onLoad,w=a.onError,x=(a.isVisible,a.noscriptSizes),z=p(a,["imgAttributes","heightInt","widthInt","qualityInt","layout","className","imgStyle","blurStyle","isLazy","placeholder","loading","srcString","config","unoptimized","loader","onLoadingCompleteRef","setBlurComplete","setIntersection","onLoad","onError","isVisible","noscriptSizes"]);return l=j?"lazy":l,h.default.createElement(h.default.Fragment,null,h.default.createElement("img",Object.assign({},z,b,{decoding:"async","data-nimg":e,className:f,style:n({},g,i),ref:h.useCallback(function(a){u(a),(null==a?void 0:a.complete)&&B(a,m,e,k,s,t)},[u,m,e,k,s,t,]),onLoad:function(a){B(a.currentTarget,m,e,k,s,t),v&&v(a)},onError:function(a){"blur"===k&&t(!0),w&&w(a)}})),(j||"blur"===k)&&h.default.createElement("noscript",null,h.default.createElement("img",Object.assign({},z,y({config:o,src:m,unoptimized:q,layout:e,width:c,quality:d,sizes:x,loader:r}),{decoding:"async","data-nimg":e,style:g,className:f,loading:l}))))};function D(a){return"/"===a[0]?a.slice(1):a}("function"==typeof b.default||"object"==typeof b.default&&null!==b.default)&& void 0===b.default.__esModule&&(Object.defineProperty(b.default,"__esModule",{value:!0}),Object.assign(b.default,b),a.exports=b.default)},7190:function(a,b,c){"use strict";Object.defineProperty(b,"__esModule",{value:!0});var d=c(4941).Z;Object.defineProperty(b,"__esModule",{value:!0}),b.useIntersection=function(a){var b=a.rootRef,c=a.rootMargin,i=a.disabled||!g,j=e.useRef(),k=d(e.useState(!1),2),l=k[0],m=k[1],n=d(e.useState(null),2),o=n[0],p=n[1];e.useEffect(function(){if(g){if(j.current&&(j.current(),j.current=void 0),!i&&!l)return o&&o.tagName&&(j.current=h(o,function(a){return a&&m(a)},{root:null==b?void 0:b.current,rootMargin:c})),function(){null==j.current||j.current(),j.current=void 0}}else if(!l){var a=f.requestIdleCallback(function(){return m(!0)});return function(){return f.cancelIdleCallback(a)}}},[o,i,c,b,l]);var q=e.useCallback(function(){m(!1)},[]);return[p,l,q]};var e=c(7294),f=c(9311),g="function"==typeof IntersectionObserver;function h(a,b,c){var d=k(c),e=d.id,f=d.observer,g=d.elements;return g.set(a,b),f.observe(a),function(){if(g.delete(a),f.unobserve(a),0===g.size){f.disconnect(),i.delete(e);var b=j.findIndex(function(a){return a.root===e.root&&a.margin===e.margin});b> -1&&j.splice(b,1)}}}var i=new Map,j=[];function k(a){var b,c={root:a.root||null,margin:a.rootMargin||""},d=j.find(function(a){return a.root===c.root&&a.margin===c.margin});if(d&&(b=i.get(d)))return b;var e=new Map,f=new IntersectionObserver(function(a){a.forEach(function(a){var b=e.get(a.target),c=a.isIntersecting||a.intersectionRatio>0;b&&c&&b(c)})},a);return b={id:c,observer:f,elements:e},j.push(c),i.set(c,b),b}("function"==typeof b.default||"object"==typeof b.default&&null!==b.default)&& void 0===b.default.__esModule&&(Object.defineProperty(b.default,"__esModule",{value:!0}),Object.assign(b.default,b),a.exports=b.default)},9008:function(a,b,c){a.exports=c(5443)},5675:function(a,b,c){a.exports=c(8045)},2703:function(a,b,c){"use strict";var d=c(414);function e(){}function f(){}f.resetWarningCache=e,a.exports=function(){function a(a,b,c,e,f,g){if(g!==d){var h=Error("Calling PropTypes validators directly is not supported by the `prop-types` package. Use PropTypes.checkPropTypes() to call them. Read more at http://fb.me/use-check-prop-types");throw h.name="Invariant Violation",h}}function b(){return a}a.isRequired=a;var c={array:a,bigint:a,bool:a,func:a,number:a,object:a,string:a,symbol:a,any:a,arrayOf:b,element:a,elementType:a,instanceOf:b,node:a,objectOf:b,oneOf:b,oneOfType:b,shape:b,exact:b,checkPropTypes:f,resetWarningCache:e};return c.PropTypes=c,c}},5697:function(a,b,c){a.exports=c(2703)()},414:function(a){"use strict";a.exports="SECRET_DO_NOT_PASS_THIS_OR_YOU_WILL_BE_FIRED"},7977:function(a,b,c){"use strict";var d=c(4184),e=c.n(d),f=c(7294),g=c(6792),h=c(5893);let i=f.forwardRef(({bsPrefix:a,bg:b,pill:c,text:d,className:f,as:i="span",...j},k)=>{let l=(0,g.vE)(a,"badge");return(0,h.jsx)(i,{ref:k,...j,className:e()(f,l,c&&"rounded-pill",d&&`text-${d}`,b&&`bg-${b}`)})});i.displayName="Badge",i.defaultProps={bg:"primary",pill:!1},b.Z=i},5005:function(a,b,c){"use strict";var d=c(4184),e=c.n(d),f=c(7294),g=c(861),h=c(6792),i=c(5893);let j=f.forwardRef(({as:a,bsPrefix:b,variant:c,size:d,active:f,className:j,...k},l)=>{let m=(0,h.vE)(b,"btn"),[n,{tagName:o}]=(0,g.FT)({tagName:a,...k});return(0,i.jsx)(o,{...n,...k,ref:l,className:e()(j,m,f&&"active",c&&`${m}-${c}`,d&&`${m}-${d}`,k.href&&k.disabled&&"disabled")})});j.displayName="Button",j.defaultProps={variant:"primary",active:!1,disabled:!1},b.Z=j},545:function(a,b,c){"use strict";c.d(b,{Z:function(){return y}});var d=c(4184),e=c.n(d),f=c(7294),g=c(6792),h=c(6611),i=c(5893),j=a=>f.forwardRef((b,c)=>(0,i.jsx)("div",{...b,ref:c,className:e()(b.className,a)}));let k=f.forwardRef(({bsPrefix:a,className:b,variant:c,as:d="img",...f},h)=>{let j=(0,g.vE)(a,"card-img");return(0,i.jsx)(d,{ref:h,className:e()(c?`${j}-${c}`:j,b),...f})});k.displayName="CardImg";let l=f.createContext(null);l.displayName="CardHeaderContext";var m=l;let n=f.forwardRef(({bsPrefix:a,className:b,as:c="div",...d},h)=>{let j=(0,g.vE)(a,"card-header"),k=(0,f.useMemo)(()=>({cardHeaderBsPrefix:j}),[j]);return(0,i.jsx)(m.Provider,{value:k,children:(0,i.jsx)(c,{ref:h,...d,className:e()(b,j)})})});n.displayName="CardHeader";let o=j("h5"),p=j("h6"),q=(0,h.Z)("card-body"),r=(0,h.Z)("card-title",{Component:o}),s=(0,h.Z)("card-subtitle",{Component:p}),t=(0,h.Z)("card-link",{Component:"a"}),u=(0,h.Z)("card-text",{Component:"p"}),v=(0,h.Z)("card-footer"),w=(0,h.Z)("card-img-overlay"),x=f.forwardRef(({bsPrefix:a,className:b,bg:c,text:d,border:f,body:h,children:j,as:k="div",...l},m)=>{let n=(0,g.vE)(a,"card");return(0,i.jsx)(k,{ref:m,...l,className:e()(b,n,c&&`bg-${c}`,d&&`text-${d}`,f&&`border-${f}`),children:h?(0,i.jsx)(q,{children:j}):j})});x.displayName="Card",x.defaultProps={body:!1};var y=Object.assign(x,{Img:k,Title:r,Subtitle:s,Body:q,Link:t,Text:u,Header:n,Footer:v,ImgOverlay:w})},7337:function(a,b,c){"use strict";var d=c(6611);b.Z=(0,d.Z)("card-group")},1555:function(a,b,c){"use strict";var d=c(4184),e=c.n(d),f=c(7294),g=c(6792),h=c(5893);let i=f.forwardRef((a,b)=>{let[{className:c,...d},{as:f="div",bsPrefix:i,spans:j}]=function({as:a,bsPrefix:b,className:c,...d}){b=(0,g.vE)(b,"col");let f=(0,g.pi)(),h=[],i=[];return f.forEach(a=>{let c=d[a];delete d[a];let e,f,g;"object"==typeof c&&null!=c?{span:e,offset:f,order:g}=c:e=c;let j="xs"!==a?`-${a}`:"";e&&h.push(!0===e?`${b}${j}`:`${b}${j}-${e}`),null!=g&&i.push(`order${j}-${g}`),null!=f&&i.push(`offset${j}-${f}`)}),[{...d,className:e()(c,...h,...i)},{as:a,bsPrefix:b,spans:h}]}(a);return(0,h.jsx)(f,{...d,ref:b,className:e()(c,!j.length&&i)})});i.displayName="Col",b.Z=i},682:function(a,b,c){"use strict";var d=c(4184),e=c.n(d),f=c(7294),g=c(6792),h=c(5893);let i=f.forwardRef(({bsPrefix:a,fluid:b,as:c="div",className:d,...f},i)=>{let j=(0,g.vE)(a,"container"),k="string"==typeof b?`-${b}`:"-fluid";return(0,h.jsx)(c,{ref:i,...f,className:e()(d,b?`${j}${k}`:j)})});i.displayName="Container",i.defaultProps={fluid:!1},b.Z=i},9301:function(a,b,c){"use strict";c.d(b,{Z:function(){return I}});var d=c(4184),e=c.n(d),f=c(5697),g=c.n(f),h=c(7294),i=c(5893);let j={type:g().string,tooltip:g().bool,as:g().elementType},k=h.forwardRef(({as:a="div",className:b,type:c="valid",tooltip:d=!1,...f},g)=>(0,i.jsx)(a,{...f,ref:g,className:e()(b,`${c}-${d?"tooltip":"feedback"}`)}));k.displayName="Feedback",k.propTypes=j;var l=k,m=c(6558),n=c(1377),o=c(6792);let p=h.forwardRef(({bsPrefix:a,className:b,htmlFor:c,...d},f)=>{let{controlId:g}=(0,h.useContext)(n.Z);return a=(0,o.vE)(a,"form-check-label"),(0,i.jsx)("label",{...d,ref:f,htmlFor:c||g,className:e()(b,a)})});p.displayName="FormCheckLabel";var q=p;let r=h.forwardRef(({id:a,bsPrefix:b,bsSwitchPrefix:c,inline:d=!1,disabled:f=!1,isValid:g=!1,isInvalid:j=!1,feedbackTooltip:k=!1,feedback:p,feedbackType:r,className:s,style:t,title:u="",type:v="checkbox",label:w,children:x,as:y="input",...z},A)=>{var B,C;b=(0,o.vE)(b,"form-check"),c=(0,o.vE)(c,"form-switch");let{controlId:D}=(0,h.useContext)(n.Z),E=(0,h.useMemo)(()=>({controlId:a||D}),[D,a]),F=!x&&null!=w&& !1!==w||(B=x,C=q,h.Children.toArray(B).some(a=>h.isValidElement(a)&&a.type===C)),G=(0,i.jsx)(m.Z,{...z,type:"switch"===v?"checkbox":v,ref:A,isValid:g,isInvalid:j,disabled:f,as:y});return(0,i.jsx)(n.Z.Provider,{value:E,children:(0,i.jsx)("div",{style:t,className:e()(s,F&&b,d&&`${b}-inline`,"switch"===v&&c),children:x||(0,i.jsxs)(i.Fragment,{children:[G,F&&(0,i.jsx)(q,{title:u,children:w}),p&&(0,i.jsx)(l,{type:r,tooltip:k,children:p})]})})})});r.displayName="FormCheck";var s=Object.assign(r,{Input:m.Z,Label:q});c(2473);let t=h.forwardRef(({bsPrefix:a,type:b,size:c,htmlSize:d,id:f,className:g,isValid:j=!1,isInvalid:k=!1,plaintext:l,readOnly:m,as:p="input",...q},r)=>{let{controlId:s}=(0,h.useContext)(n.Z);a=(0,o.vE)(a,"form-control");let t;return t=l?{[`${a}-plaintext`]:!0}:{[a]:!0,[`${a}-${c}`]:c},(0,i.jsx)(p,{...q,type:b,size:d,ref:r,readOnly:m,id:f||s,className:e()(g,t,j&&"is-valid",k&&"is-invalid","color"===b&&`${a}-color`)})});t.displayName="FormControl";var u=Object.assign(t,{Feedback:l}),v=(0,c(6611).Z)("form-floating");let w=h.forwardRef(({controlId:a,as:b="div",...c},d)=>{let e=(0,h.useMemo)(()=>({controlId:a}),[a]);return(0,i.jsx)(n.Z.Provider,{value:e,children:(0,i.jsx)(b,{...c,ref:d})})});w.displayName="FormGroup";var x=w,y=c(1555);let z=h.forwardRef(({as:a="label",bsPrefix:b,column:c,visuallyHidden:d,className:f,htmlFor:g,...j},k)=>{let{controlId:l}=(0,h.useContext)(n.Z);b=(0,o.vE)(b,"form-label");let m="col-form-label";"string"==typeof c&&(m=`${m} ${m}-${c}`);let p=e()(f,b,d&&"visually-hidden",c&&m);return(g=g||l,c)?(0,i.jsx)(y.Z,{ref:k,as:"label",className:p,htmlFor:g,...j}):(0,i.jsx)(a,{ref:k,className:p,htmlFor:g,...j})});z.displayName="FormLabel",z.defaultProps={column:!1,visuallyHidden:!1};let A=h.forwardRef(({bsPrefix:a,className:b,id:c,...d},f)=>{let{controlId:g}=(0,h.useContext)(n.Z);return a=(0,o.vE)(a,"form-range"),(0,i.jsx)("input",{...d,type:"range",ref:f,className:e()(b,a),id:c||g})});A.displayName="FormRange";let B=h.forwardRef(({bsPrefix:a,size:b,htmlSize:c,className:d,isValid:f=!1,isInvalid:g=!1,id:j,...k},l)=>{let{controlId:m}=(0,h.useContext)(n.Z);return a=(0,o.vE)(a,"form-select"),(0,i.jsx)("select",{...k,size:c,ref:l,className:e()(d,a,b&&`${a}-${b}`,f&&"is-valid",g&&"is-invalid"),id:j||m})});B.displayName="FormSelect";let C=h.forwardRef(({bsPrefix:a,className:b,as:c="small",muted:d,...f},g)=>(a=(0,o.vE)(a,"form-text"),(0,i.jsx)(c,{...f,ref:g,className:e()(b,a,d&&"text-muted")})));C.displayName="FormText";let D=h.forwardRef((a,b)=>(0,i.jsx)(s,{...a,ref:b,type:"switch"}));D.displayName="Switch";var E=Object.assign(D,{Input:s.Input,Label:s.Label});let F=h.forwardRef(({bsPrefix:a,className:b,children:c,controlId:d,label:f,...g},h)=>(a=(0,o.vE)(a,"form-floating"),(0,i.jsxs)(x,{ref:h,className:e()(b,a),controlId:d,...g,children:[c,(0,i.jsx)("label",{htmlFor:d,children:f})]})));F.displayName="FloatingLabel";let G={_ref:g().any,validated:g().bool,as:g().elementType},H=h.forwardRef(({className:a,validated:b,as:c="form",...d},f)=>(0,i.jsx)(c,{...d,ref:f,className:e()(a,b&&"was-validated")}));H.displayName="Form",H.propTypes=G;var I=Object.assign(H,{Group:x,Control:u,Floating:v,Check:s,Switch:E,Label:z,Text:C,Range:A,Select:B,FloatingLabel:F})},6558:function(a,b,c){"use strict";var d=c(4184),e=c.n(d),f=c(7294),g=c(1377),h=c(6792),i=c(5893);let j=f.forwardRef(({id:a,bsPrefix:b,className:c,type:d="checkbox",isValid:j=!1,isInvalid:k=!1,as:l="input",...m},n)=>{let{controlId:o}=(0,f.useContext)(g.Z);return b=(0,h.vE)(b,"form-check-input"),(0,i.jsx)(l,{...m,ref:n,type:d,id:a||o,className:e()(c,b,j&&"is-valid",k&&"is-invalid")})});j.displayName="FormCheckInput",b.Z=j},1377:function(a,b,c){"use strict";var d=c(7294);let e=d.createContext({});b.Z=e},1604:function(a,b,c){"use strict";c.d(b,{Z:function(){return q}});var d=c(4184),e=c.n(d),f=c(7294),g=c(6611),h=c(6792),i=c(6558);let j=f.createContext(null);j.displayName="InputGroupContext";var k=j,l=c(5893);let m=(0,g.Z)("input-group-text",{Component:"span"}),n=a=>(0,l.jsx)(m,{children:(0,l.jsx)(i.Z,{type:"checkbox",...a})}),o=a=>(0,l.jsx)(m,{children:(0,l.jsx)(i.Z,{type:"radio",...a})}),p=f.forwardRef(({bsPrefix:a,size:b,hasValidation:c,className:d,as:g="div",...i},j)=>{a=(0,h.vE)(a,"input-group");let m=(0,f.useMemo)(()=>({}),[]);return(0,l.jsx)(k.Provider,{value:m,children:(0,l.jsx)(g,{ref:j,...i,className:e()(d,a,b&&`${a}-${b}`,c&&"has-validation")})})});p.displayName="InputGroup";var q=Object.assign(p,{Text:m,Radio:o,Checkbox:n})},6149:function(a,b,c){"use strict";c.d(b,{Z:function(){return J}});var d=c(4184),e=c.n(d),f=c(7294);function g(){return(g=Object.assign?Object.assign.bind():function(a){for(var b=1;b<arguments.length;b++){var c=arguments[b];for(var d in c)Object.prototype.hasOwnProperty.call(c,d)&&(a[d]=c[d])}return a}).apply(this,arguments)}c(2473),c(1143);function h(a){return"default"+a.charAt(0).toUpperCase()+a.substr(1)}function i(a){var b=j(a,"string");return"symbol"==typeof b?b:String(b)}function j(a,b){if("object"!=typeof a||null===a)return a;var c=a[Symbol.toPrimitive];if(void 0!==c){var d=c.call(a,b||"default");if("object"!=typeof d)return d;throw TypeError("@@toPrimitive must return a primitive value.")}return("string"===b?String:Number)(a)}(function a(){var b=this.constructor.getDerivedStateFromProps(this.props,this.state);null!=b&&this.setState(b)}).__suppressDeprecationWarning=!0,(function a(b){this.setState((function(a){var c=this.constructor.getDerivedStateFromProps(b,a);return null!=c?c:null}).bind(this))}).__suppressDeprecationWarning=!0,(function a(b,c){try{var d=this.props,e=this.state;this.props=b,this.state=c,this.__reactInternalSnapshotFlag=!0,this.__reactInternalSnapshot=this.getSnapshotBeforeUpdate(d,e)}finally{this.props=d,this.state=e}}).__suppressDeprecationWarning=!0;var k=Function.prototype.bind.call(Function.prototype.call,[].slice),l=function(a){return a&&"function"!=typeof a?function(b){a.current=b}:a};let m=f.createContext(null);m.displayName="NavContext";var n=m;let o=f.createContext(null),p=(a,b=null)=>null!=a?String(a):b||null;var q=o;let r=f.createContext(null);var s=r;function t(a){return`data-rr-ui-${a}`}var u=function(a){var b=(0,f.useRef)(a);return(0,f.useEffect)(function(){b.current=a},[a]),b};function v(a){var b=u(a);return(0,f.useCallback)(function(){return b.current&&b.current.apply(b,arguments)},[b])}var w=c(861),x=c(5893);let y=["as","active","eventKey"];function z({key:a,onClick:b,active:c,id:d,role:e,disabled:g}){let h=(0,f.useContext)(q),i=(0,f.useContext)(n),j=(0,f.useContext)(s),k=c,l={role:e};if(i){e||"tablist"!==i.role||(l.role="tab");let m=i.getControllerId(null!=a?a:null),o=i.getControlledId(null!=a?a:null);l[t("event-key")]=a,l.id=m||d,((k=null==c&&null!=a?i.activeKey===a:c)|| !(null!=j&&j.unmountOnExit)&&!(null!=j&&j.mountOnEnter))&&(l["aria-controls"]=o)}return"tab"===l.role&&(l["aria-selected"]=k,k||(l.tabIndex=-1),g&&(l.tabIndex=-1,l["aria-disabled"]=!0)),l.onClick=v(c=>{!g&&(null==b||b(c),null!=a&&h&&!c.isPropagationStopped()&&h(a,c))}),[l,{isActive:k}]}let A=f.forwardRef((a,b)=>{let{as:c=w.ZP,active:d,eventKey:e}=a,f=function(a,b){if(null==a)return{};var c,d,e={},f=Object.keys(a);for(d=0;d<f.length;d++)c=f[d],b.indexOf(c)>=0||(e[c]=a[c]);return e}(a,y),[g,h]=z(Object.assign({key:p(e,f.href),active:d},f));return g[t("active")]=h.isActive,(0,x.jsx)(c,Object.assign({},f,g,{ref:b}))});A.displayName="NavItem";let B=["as","onSelect","activeKey","role","onKeyDown"],C=()=>{},D=t("event-key"),E=f.forwardRef((a,b)=>{var c,d;let{as:e="div",onSelect:g,activeKey:h,role:i,onKeyDown:j}=a,m=function(a,b){if(null==a)return{};var c,d,e={},f=Object.keys(a);for(d=0;d<f.length;d++)c=f[d],b.indexOf(c)>=0||(e[c]=a[c]);return e}(a,B),o=(0,f.useReducer)(function(a){return!a},!1)[1],r=(0,f.useRef)(!1),t=(0,f.useContext)(q),u=(0,f.useContext)(s),v,w;u&&(i=i||"tablist",h=u.activeKey,v=u.getControlledId,w=u.getControllerId);let y=(0,f.useRef)(null),z=a=>{var b,c;let d=y.current;if(!d)return null;let e=(b=d,c=`[${D}]:not([aria-disabled=true])`,k(b.querySelectorAll(c))),f=d.querySelector("[aria-selected=true]");if(!f||f!==document.activeElement)return null;let g=e.indexOf(f);if(-1===g)return null;let h=g+a;return h>=e.length&&(h=0),h<0&&(h=e.length-1),e[h]},A=(a,b)=>{null!=a&&(null==g||g(a,b),null==t||t(a,b))},E=a=>{if(null==j||j(a),!u)return;let b;switch(a.key){case"ArrowLeft":case"ArrowUp":b=z(-1);break;case"ArrowRight":case"ArrowDown":b=z(1);break;default:return}if(b){var c;a.preventDefault(),A(b.dataset[`rrUi${c="EventKey"}`]||null,a),r.current=!0,o()}};(0,f.useEffect)(()=>{if(y.current&&r.current){let a=y.current.querySelector(`[${D}][aria-selected=true]`);null==a||a.focus()}r.current=!1});let F=(c=b,d=y,(0,f.useMemo)(function(){var a,b,e,f;return a=c,b=d,e=l(a),f=l(b),function(a){e&&e(a),f&&f(a)}},[c,d]));return(0,x.jsx)(q.Provider,{value:A,children:(0,x.jsx)(n.Provider,{value:{role:i,activeKey:p(h),getControlledId:v||C,getControllerId:w||C},children:(0,x.jsx)(e,Object.assign({},m,{onKeyDown:E,ref:F,role:i}))})})});E.displayName="Nav";var F=Object.assign(E,{Item:A}),G=c(6792);let H=f.forwardRef(({bsPrefix:a,active:b,disabled:c,eventKey:d,className:f,variant:g,action:h,as:i,...j},k)=>{a=(0,G.vE)(a,"list-group-item");let[l,m]=z({key:p(d,j.href),active:b,...j}),n=v(a=>{if(c){a.preventDefault(),a.stopPropagation();return}l.onClick(a)});c&& void 0===j.tabIndex&&(j.tabIndex=-1,j["aria-disabled"]=!0);let o=i||(h?j.href?"a":"button":"div");return(0,x.jsx)(o,{ref:k,...j,...l,onClick:n,className:e()(f,a,m.isActive&&"active",c&&"disabled",g&&`${a}-${g}`,h&&`${a}-action`)})});H.displayName="ListGroupItem";let I=f.forwardRef((a,b)=>{var c,d;let{className:j,bsPrefix:k,variant:l,horizontal:m,numbered:n,as:o="div",...p}=(c=a,Object.keys(d={activeKey:"onSelect"}).reduce(function(a,b){var e,j,k,l,m,n,o,p,q,r,s=a,t=s[h(b)],u=s[b],v=function(a,b){if(null==a)return{};var c,d,e={},f=Object.keys(a);for(d=0;d<f.length;d++)c=f[d],b.indexOf(c)>=0||(e[c]=a[c]);return e}(s,[h(b),b].map(i)),w=d[b],x=(j=u,k=t,l=c[w],m=(0,f.useRef)(void 0!==j),n=(0,f.useState)(k),o=n[0],p=n[1],q=void 0!==j,r=m.current,m.current=q,!q&&r&&o!==k&&p(k),[q?j:o,(0,f.useCallback)(function(a){for(var b=arguments.length,c=Array(b>1?b-1:0),d=1;d<b;d++)c[d-1]=arguments[d];l&&l.apply(void 0,[a].concat(c)),p(a)},[l])]),y=x[0],z=x[1];return g({},v,((e={})[b]=y,e[w]=z,e))},c)),q=(0,G.vE)(k,"list-group"),r;return m&&(r=!0===m?"horizontal":`horizontal-${m}`),(0,x.jsx)(F,{ref:b,...p,as:o,className:e()(j,q,l&&`${q}-${l}`,r&&`${q}-${r}`,n&&`${q}-numbered`)})});I.displayName="ListGroup";var J=Object.assign(I,{Item:H})},1608:function(a,b,c){"use strict";var d=c(4184),e=c.n(d),f=c(7294),g=c(6792),h=c(5893);let i=f.forwardRef(({bsPrefix:a,className:b,as:c="div",...d},f)=>{let i=(0,g.vE)(a,"row"),j=(0,g.pi)(),k=`${i}-cols`,l=[];return j.forEach(a=>{let b=d[a];delete d[a];let c;null!=b&&"object"==typeof b?{cols:c}=b:c=b;let e="xs"!==a?`-${a}`:"";null!=c&&l.push(`${k}${e}-${c}`)}),(0,h.jsx)(c,{ref:f,...d,className:e()(b,i,...l)})});i.displayName="Row",b.Z=i},478:function(a,b,c){"use strict";c.d(b,{Z:function(){return j}});var d=c(4184),e=c.n(d),f=c(7294),g=c(6792),h=c(5893);let i=f.forwardRef(({as:a="div",bsPrefix:b,className:c,direction:d,gap:f,...i},j)=>{b=(0,g.vE)(b,"horizontal"===d?"hstack":"vstack");let k=(0,g.pi)();return(0,h.jsx)(a,{...i,ref:j,className:e()(c,b,...function(a,b=g.Hz){let c=[];return Object.entries(a).forEach(([a,d])=>{null!=d&&("object"==typeof d?b.forEach(b=>{let e=d[b];if(null!=e){let f="xs"!==b?`-${b}`:"";c.push(`${a}${f}-${e}`)}}):c.push(`${a}-${d}`))}),c}({gap:f,breakpoints:k}))})});i.displayName="Stack";var j=i},6792:function(a,b,c){"use strict";c.d(b,{Hz:function(){return e},pi:function(){return j},vE:function(){return i}});var d=c(7294);c(5893);let e=["xxl","xl","lg","md","sm","xs"],f=d.createContext({prefixes:{},breakpoints:e}),{Consumer:g,Provider:h}=f;function i(a,b){let{prefixes:c}=(0,d.useContext)(f);return a||c[b]||b}function j(){let{breakpoints:a}=(0,d.useContext)(f);return a}},6611:function(a,b,c){"use strict";c.d(b,{Z:function(){return k}});var d=c(4184),e=c.n(d),f=/-(.)/g,g=c(7294),h=c(6792),i=c(5893);let j=a=>{var b;return a[0].toUpperCase()+(b=a).replace(f,function(a,b){return b.toUpperCase()}).slice(1)};function k(a,{displayName:b=j(a),Component:c,defaultProps:d}={}){let f=g.forwardRef(({className:b,bsPrefix:d,as:f=c||"div",...g},j)=>{let k=(0,h.vE)(d,a);return(0,i.jsx)(f,{ref:j,className:e()(b,k),...g})});return f.defaultProps=d,f.displayName=b,f}},2473:function(a){"use strict";var b=function(){};a.exports=b}}])