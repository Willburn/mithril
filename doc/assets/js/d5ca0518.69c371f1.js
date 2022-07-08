"use strict";(self.webpackChunkmithril_doc=self.webpackChunkmithril_doc||[]).push([[4701],{3905:function(e,t,r){r.d(t,{Zo:function(){return l},kt:function(){return d}});var n=r(7294);function i(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function o(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function a(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?o(Object(r),!0).forEach((function(t){i(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):o(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function c(e,t){if(null==e)return{};var r,n,i=function(e,t){if(null==e)return{};var r,n,i={},o=Object.keys(e);for(n=0;n<o.length;n++)r=o[n],t.indexOf(r)>=0||(i[r]=e[r]);return i}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(n=0;n<o.length;n++)r=o[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(i[r]=e[r])}return i}var u=n.createContext({}),s=function(e){var t=n.useContext(u),r=t;return e&&(r="function"==typeof e?e(t):a(a({},t),e)),r},l=function(e){var t=s(e.components);return n.createElement(u.Provider,{value:t},e.children)},p={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},g=n.forwardRef((function(e,t){var r=e.components,i=e.mdxType,o=e.originalType,u=e.parentName,l=c(e,["components","mdxType","originalType","parentName"]),g=s(r),d=i,h=g["".concat(u,".").concat(d)]||g[d]||p[d]||o;return r?n.createElement(h,a(a({ref:t},l),{},{components:r})):n.createElement(h,a({ref:t},l))}));function d(e,t){var r=arguments,i=t&&t.mdxType;if("string"==typeof e||i){var o=r.length,a=new Array(o);a[0]=g;var c={};for(var u in t)hasOwnProperty.call(t,u)&&(c[u]=t[u]);c.originalType=e,c.mdxType="string"==typeof e?e:i,a[1]=c;for(var s=2;s<o;s++)a[s]=r[s];return n.createElement.apply(null,a)}return n.createElement.apply(null,r)}g.displayName="MDXCreateElement"},1527:function(e,t,r){r.r(t),r.d(t,{assets:function(){return l},contentTitle:function(){return u},default:function(){return d},frontMatter:function(){return c},metadata:function(){return s},toc:function(){return p}});var n=r(7462),i=r(3366),o=(r(7294),r(3905)),a=["components"],c={sidebar_position:2},u="Mithril Aggregator Node",s={unversionedId:"manual/developer-docs/architecture/aggregator",id:"manual/developer-docs/architecture/aggregator",title:"Mithril Aggregator Node",description:"The Aggregator is the central piece of the Mithril certificate creation.",source:"@site/root/manual/developer-docs/architecture/aggregator.md",sourceDirName:"manual/developer-docs/architecture",slug:"/manual/developer-docs/architecture/aggregator",permalink:"/doc/manual/developer-docs/architecture/aggregator",editUrl:"https://github.com/input-output-hk/mithril/tree/main/docs/root/root/manual/developer-docs/architecture/aggregator.md",tags:[],version:"current",sidebarPosition:2,frontMatter:{sidebar_position:2},sidebar:"docSideBar",previous:{title:"Architecture",permalink:"/doc/manual/developer-docs/architecture/"},next:{title:"Mithril Signer Node",permalink:"/doc/manual/developer-docs/architecture/signer"}},l={},p=[{value:"Certificate creation",id:"certificate-creation",level:2},{value:"Runtime",id:"runtime",level:2}],g={toc:p};function d(e){var t=e.components,c=(0,i.Z)(e,a);return(0,o.kt)("wrapper",(0,n.Z)({},g,c,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("h1",{id:"mithril-aggregator-node"},"Mithril Aggregator Node"),(0,o.kt)("p",null,"The Aggregator is the central piece of the Mithril certificate creation."),(0,o.kt)("p",null,"The first phase of the protocole is triggered when a new Epoch starts in the Cardano network. The aggregator node saves the current stake ditribution and starts performing the snapshot for this new epoch which is then stored in the cloud. A message is then created containing the current and the previous snapshots digests."),(0,o.kt)("h2",{id:"certificate-creation"},"Certificate creation"),(0,o.kt)("p",null,"During this phase, the aggregator waits to reach a stake quorum of signers. They register to be part of the lotery process of the multisignature. During all this time, the certificate is in ",(0,o.kt)("em",{parentName:"p"},"pending")," state. Once the quorum is reached, the aggregator issues the multisignature certificate and stores it in the certificate chain."),(0,o.kt)("p",null,(0,o.kt)("img",{src:r(9014).Z,width:"960",height:"720"})),(0,o.kt)("h2",{id:"runtime"},"Runtime"),(0,o.kt)("p",null,(0,o.kt)("img",{src:r(960).Z,width:"630",height:"1584"})))}d.isMDXComponent=!0},960:function(e,t,r){t.Z=r.p+"assets/images/aggregator-runtime-2d07673184a875e1b862f37fef68f4fb.jpg"},9014:function(e,t,r){t.Z=r.p+"assets/images/aggregator-workflow-4253d3759b87c474897cfe79dd80e0a3.png"}}]);