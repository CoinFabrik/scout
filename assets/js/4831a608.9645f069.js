"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[5405],{9613:(e,t,r)=>{r.d(t,{Zo:()=>u,kt:()=>f});var n=r(9496);function o(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function a(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function i(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?a(Object(r),!0).forEach((function(t){o(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):a(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function c(e,t){if(null==e)return{};var r,n,o=function(e,t){if(null==e)return{};var r,n,o={},a=Object.keys(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||(o[r]=e[r]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(o[r]=e[r])}return o}var l=n.createContext({}),s=function(e){var t=n.useContext(l),r=t;return e&&(r="function"==typeof e?e(t):i(i({},t),e)),r},u=function(e){var t=s(e.components);return n.createElement(l.Provider,{value:t},e.children)},p="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},m=n.forwardRef((function(e,t){var r=e.components,o=e.mdxType,a=e.originalType,l=e.parentName,u=c(e,["components","mdxType","originalType","parentName"]),p=s(r),m=o,f=p["".concat(l,".").concat(m)]||p[m]||d[m]||a;return r?n.createElement(f,i(i({ref:t},u),{},{components:r})):n.createElement(f,i({ref:t},u))}));function f(e,t){var r=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=r.length,i=new Array(a);i[0]=m;var c={};for(var l in t)hasOwnProperty.call(t,l)&&(c[l]=t[l]);c.originalType=e,c[p]="string"==typeof e?e:o,i[1]=c;for(var s=2;s<a;s++)i[s]=r[s];return n.createElement.apply(null,i)}return n.createElement.apply(null,r)}m.displayName="MDXCreateElement"},9257:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>l,contentTitle:()=>i,default:()=>d,frontMatter:()=>a,metadata:()=>c,toc:()=>s});var n=r(2564),o=(r(9496),r(9613));const a={},i="Panic error",c={unversionedId:"detectors/panic-error",id:"detectors/panic-error",title:"Panic error",description:"What it does",source:"@site/docs/detectors/4-panic-error.md",sourceDirName:"detectors",slug:"/detectors/panic-error",permalink:"/scout/docs/detectors/panic-error",draft:!1,editUrl:"https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/docs/detectors/4-panic-error.md",tags:[],version:"current",sidebarPosition:4,frontMatter:{},sidebar:"docsSidebar",previous:{title:"Reentrancy",permalink:"/scout/docs/detectors/reentrancy"},next:{title:"Unused return enum",permalink:"/scout/docs/detectors/unused-return-enum"}},l={},s=[{value:"What it does",id:"what-it-does",level:3},{value:"Why is this bad?",id:"why-is-this-bad",level:3},{value:"Known problems",id:"known-problems",level:3},{value:"Example",id:"example",level:3},{value:"Implementation",id:"implementation",level:3}],u={toc:s},p="wrapper";function d(e){let{components:t,...r}=e;return(0,o.kt)(p,(0,n.Z)({},u,r,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("h1",{id:"panic-error"},"Panic error"),(0,o.kt)("h3",{id:"what-it-does"},"What it does"),(0,o.kt)("p",null,"The panic! macro is used to stop execution when a condition is not met.\nThis is useful for testing and prototyping, but should be avoided in production code"),(0,o.kt)("h3",{id:"why-is-this-bad"},"Why is this bad?"),(0,o.kt)("p",null,"The usage of panic! is not recommended because it will stop the execution of the caller contract."),(0,o.kt)("h3",{id:"known-problems"},"Known problems"),(0,o.kt)("p",null,"While this linter detects explicit calls to panic!, there are some ways to raise a panic such as unwrap() or expect()."),(0,o.kt)("h3",{id:"example"},"Example"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},'// example code where a warning is issued\npub fn add(&mut self, value: u32)   {\n   match self.value.checked_add(value) {\n       Some(v) => self.value = v,\n       None => panic!("Overflow error"),\n   };\n}\n')),(0,o.kt)("p",null,"// example code that does not raise a warning"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},'#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]\n#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]\npub enum Error {\n  /// An overflow was produced while adding\n  OverflowError,\n}\n\npub fn add(&mut self, value: u32) -> Result<(), Error>  {\n    match self.value.checked_add(value) {\n        Some(v) => self.value = v,\n        None => return Err(Error::OverflowError),\n    };\n    Ok(())\n}\n')),(0,o.kt)("h3",{id:"implementation"},"Implementation"),(0,o.kt)("p",null,"The detector's implementation can be found at ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout/tree/main/detectors/panic-error"},"this link"),"."))}d.isMDXComponent=!0}}]);