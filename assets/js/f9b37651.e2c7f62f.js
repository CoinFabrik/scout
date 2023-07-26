"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[1612],{9613:(e,t,r)=>{r.d(t,{Zo:()=>u,kt:()=>f});var n=r(9496);function o(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function a(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function i(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?a(Object(r),!0).forEach((function(t){o(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):a(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function s(e,t){if(null==e)return{};var r,n,o=function(e,t){if(null==e)return{};var r,n,o={},a=Object.keys(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||(o[r]=e[r]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(o[r]=e[r])}return o}var c=n.createContext({}),l=function(e){var t=n.useContext(c),r=t;return e&&(r="function"==typeof e?e(t):i(i({},t),e)),r},u=function(e){var t=l(e.components);return n.createElement(c.Provider,{value:t},e.children)},m="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},p=n.forwardRef((function(e,t){var r=e.components,o=e.mdxType,a=e.originalType,c=e.parentName,u=s(e,["components","mdxType","originalType","parentName"]),m=l(r),p=o,f=m["".concat(c,".").concat(p)]||m[p]||d[p]||a;return r?n.createElement(f,i(i({ref:t},u),{},{components:r})):n.createElement(f,i({ref:t},u))}));function f(e,t){var r=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=r.length,i=new Array(a);i[0]=p;var s={};for(var c in t)hasOwnProperty.call(t,c)&&(s[c]=t[c]);s.originalType=e,s[m]="string"==typeof e?e:o,i[1]=s;for(var l=2;l<a;l++)i[l]=r[l];return n.createElement.apply(null,i)}return n.createElement.apply(null,r)}p.displayName="MDXCreateElement"},8492:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>c,contentTitle:()=>i,default:()=>d,frontMatter:()=>a,metadata:()=>s,toc:()=>l});var n=r(2564),o=(r(9496),r(9613));const a={},i="Avoid fromat! macro usage",s={unversionedId:"detectors/avoid-format-string",id:"detectors/avoid-format-string",title:"Avoid fromat! macro usage",description:"What it does",source:"@site/docs/detectors/17-avoid-format-string.md",sourceDirName:"detectors",slug:"/detectors/avoid-format-string",permalink:"/scout/docs/detectors/avoid-format-string",draft:!1,editUrl:"https://github.com/CoinFabrik/scout/docs/detectors/17-avoid-format-string.md",tags:[],version:"current",sidebarPosition:17,frontMatter:{},sidebar:"docsSidebar",previous:{title:"Avoid core::mem::forget usage",permalink:"/scout/docs/detectors/avoid-core-mem-forget"},next:{title:"Iterators over indexing",permalink:"/scout/docs/detectors/iterators-over-indexing"}},c={},l=[{value:"What it does",id:"what-it-does",level:3},{value:"Why is this bad?",id:"why-is-this-bad",level:3},{value:"Example",id:"example",level:3},{value:"Implementation",id:"implementation",level:3}],u={toc:l},m="wrapper";function d(e){let{components:t,...r}=e;return(0,o.kt)(m,(0,n.Z)({},u,r,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("h1",{id:"avoid-fromat-macro-usage"},"Avoid fromat! macro usage"),(0,o.kt)("h3",{id:"what-it-does"},"What it does"),(0,o.kt)("p",null,"Checks for ",(0,o.kt)("inlineCode",{parentName:"p"},"format!")," macro usage."),(0,o.kt)("h3",{id:"why-is-this-bad"},"Why is this bad?"),(0,o.kt)("p",null,"The usage of format! is not recommended because it can panic the execution."),(0,o.kt)("h3",{id:"example"},"Example"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},'    #[ink(message)]\n    pub fn crash(&self) -> Result<(), Error> {\n        Err(Error::FormatError {\n            msg: (format!("{}", self.value)),\n        })\n    }\n')),(0,o.kt)("p",null,"Use instead:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},"    pub enum Error {\n        FormatError { msg: String },\n        CrashError\n    }\n\n    #[ink(message)]\n    pub fn crash(&self) -> Result<(), Error> {\n        Err(Error::FormatError { msg: self.value.to_string() })\n    }\n")),(0,o.kt)("h3",{id:"implementation"},"Implementation"),(0,o.kt)("p",null,"The detector's implementation can be found at ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout/tree/main/detectors/avoid-format!-string"},"this link"),"."))}d.isMDXComponent=!0}}]);