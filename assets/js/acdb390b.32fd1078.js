"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[2680],{9613:(e,t,r)=>{r.d(t,{Zo:()=>u,kt:()=>f});var n=r(9496);function a(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function o(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function i(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?o(Object(r),!0).forEach((function(t){a(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):o(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function s(e,t){if(null==e)return{};var r,n,a=function(e,t){if(null==e)return{};var r,n,a={},o=Object.keys(e);for(n=0;n<o.length;n++)r=o[n],t.indexOf(r)>=0||(a[r]=e[r]);return a}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(n=0;n<o.length;n++)r=o[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(a[r]=e[r])}return a}var l=n.createContext({}),c=function(e){var t=n.useContext(l),r=t;return e&&(r="function"==typeof e?e(t):i(i({},t),e)),r},u=function(e){var t=c(e.components);return n.createElement(l.Provider,{value:t},e.children)},p="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},m=n.forwardRef((function(e,t){var r=e.components,a=e.mdxType,o=e.originalType,l=e.parentName,u=s(e,["components","mdxType","originalType","parentName"]),p=c(r),m=a,f=p["".concat(l,".").concat(m)]||p[m]||d[m]||o;return r?n.createElement(f,i(i({ref:t},u),{},{components:r})):n.createElement(f,i({ref:t},u))}));function f(e,t){var r=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var o=r.length,i=new Array(o);i[0]=m;var s={};for(var l in t)hasOwnProperty.call(t,l)&&(s[l]=t[l]);s.originalType=e,s[p]="string"==typeof e?e:a,i[1]=s;for(var c=2;c<o;c++)i[c]=r[c];return n.createElement.apply(null,i)}return n.createElement.apply(null,r)}m.displayName="MDXCreateElement"},9457:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>l,contentTitle:()=>i,default:()=>d,frontMatter:()=>o,metadata:()=>s,toc:()=>c});var n=r(2564),a=(r(9496),r(9613));const o={},i="Unrestricted Transfer From",s={unversionedId:"detectors/assert-violation",id:"detectors/assert-violation",title:"Unrestricted Transfer From",description:"What it does",source:"@site/docs/detectors/15-assert-violation.md",sourceDirName:"detectors",slug:"/detectors/assert-violation",permalink:"/scout/docs/detectors/assert-violation",draft:!1,editUrl:"https://github.com/CoinFabrik/scout/docs/detectors/15-assert-violation.md",tags:[],version:"current",sidebarPosition:15,frontMatter:{},sidebar:"docsSidebar",previous:{title:"Unrestricted Transfer From",permalink:"/scout/docs/detectors/unrestricted-transfer-from"},next:{title:"Contribute",permalink:"/scout/docs/contribute"}},l={},c=[{value:"What it does",id:"what-it-does",level:3},{value:"Why is this bad?",id:"why-is-this-bad",level:3},{value:"Example",id:"example",level:3},{value:"Implementation",id:"implementation",level:3}],u={toc:c},p="wrapper";function d(e){let{components:t,...r}=e;return(0,a.kt)(p,(0,n.Z)({},u,r,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h1",{id:"unrestricted-transfer-from"},"Unrestricted Transfer From"),(0,a.kt)("h3",{id:"what-it-does"},"What it does"),(0,a.kt)("p",null,"Checks for ",(0,a.kt)("inlineCode",{parentName:"p"},"assert!")," macro usage."),(0,a.kt)("h3",{id:"why-is-this-bad"},"Why is this bad?"),(0,a.kt)("p",null,"The ",(0,a.kt)("inlineCode",{parentName:"p"},"assert!")," macro can cause the contract to panic. "),(0,a.kt)("h3",{id:"example"},"Example"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},'    #[ink(message)]\n    pub fn assert_if_greater_than_10(&self, value: u128) -> bool {\n        assert!(value <= 10, "value should be less than 10");\n        true\n    }\n')),(0,a.kt)("p",null,"Use instead:"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},'    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]\n    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]\n    pub enum Error {\n        GreaterThan10,\n    }\n\n    #[ink(message)]\n    pub fn revert_if_greater_than_10(&self, value: u128) -> Result<bool, Error> {\n        if value <= 10 {\n            return Ok(true)\n        } else {\n            return Err(Error::GreaterThan10)\n        }\n    }\n')),(0,a.kt)("h3",{id:"implementation"},"Implementation"),(0,a.kt)("p",null,"The detector's implementation can be found at ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout/tree/main/detectors/assert-violation"},"this link"),"."))}d.isMDXComponent=!0}}]);