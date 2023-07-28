"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[907],{9613:(e,t,r)=>{r.d(t,{Zo:()=>u,kt:()=>f});var n=r(9496);function i(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function a(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function o(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?a(Object(r),!0).forEach((function(t){i(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):a(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function l(e,t){if(null==e)return{};var r,n,i=function(e,t){if(null==e)return{};var r,n,i={},a=Object.keys(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||(i[r]=e[r]);return i}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(i[r]=e[r])}return i}var s=n.createContext({}),c=function(e){var t=n.useContext(s),r=t;return e&&(r="function"==typeof e?e(t):o(o({},t),e)),r},u=function(e){var t=c(e.components);return n.createElement(s.Provider,{value:t},e.children)},m="mdxType",p={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},d=n.forwardRef((function(e,t){var r=e.components,i=e.mdxType,a=e.originalType,s=e.parentName,u=l(e,["components","mdxType","originalType","parentName"]),m=c(r),d=i,f=m["".concat(s,".").concat(d)]||m[d]||p[d]||a;return r?n.createElement(f,o(o({ref:t},u),{},{components:r})):n.createElement(f,o({ref:t},u))}));function f(e,t){var r=arguments,i=t&&t.mdxType;if("string"==typeof e||i){var a=r.length,o=new Array(a);o[0]=d;var l={};for(var s in t)hasOwnProperty.call(t,s)&&(l[s]=t[s]);l.originalType=e,l[m]="string"==typeof e?e:i,o[1]=l;for(var c=2;c<a;c++)o[c]=r[c];return n.createElement.apply(null,o)}return n.createElement.apply(null,r)}d.displayName="MDXCreateElement"},7088:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>s,contentTitle:()=>o,default:()=>p,frontMatter:()=>a,metadata:()=>l,toc:()=>c});var n=r(2564),i=(r(9496),r(9613));const a={},o="Avoid fromat! macro usage",l={unversionedId:"vulnerabilities/avoid-format-string",id:"vulnerabilities/avoid-format-string",title:"Avoid fromat! macro usage",description:"Description",source:"@site/docs/vulnerabilities/17-avoid-format-string.md",sourceDirName:"vulnerabilities",slug:"/vulnerabilities/avoid-format-string",permalink:"/scout/docs/vulnerabilities/avoid-format-string",draft:!1,editUrl:"https://github.com/CoinFabrik/scout/docs/vulnerabilities/17-avoid-format-string.md",tags:[],version:"current",sidebarPosition:17,frontMatter:{},sidebar:"docsSidebar",previous:{title:"Avoid core::mem::forget usage",permalink:"/scout/docs/vulnerabilities/avoid-core-mem-forget"},next:{title:"Unprotected Self Destruct",permalink:"/scout/docs/vulnerabilities/unprotected-self-destruct"}},s={},c=[{value:"Description",id:"description",level:2},{value:"Exploit Scenario",id:"exploit-scenario",level:2},{value:"Remediation",id:"remediation",level:2},{value:"References",id:"references",level:2}],u={toc:c},m="wrapper";function p(e){let{components:t,...r}=e;return(0,i.kt)(m,(0,n.Z)({},u,r,{components:t,mdxType:"MDXLayout"}),(0,i.kt)("h1",{id:"avoid-fromat-macro-usage"},"Avoid fromat! macro usage"),(0,i.kt)("h2",{id:"description"},"Description"),(0,i.kt)("ul",null,(0,i.kt)("li",{parentName:"ul"},"Vulnerability Category: ",(0,i.kt)("inlineCode",{parentName:"li"},"Validations and error handling")),(0,i.kt)("li",{parentName:"ul"},"Vulnerability Severity: ",(0,i.kt)("inlineCode",{parentName:"li"},"Enhacement")),(0,i.kt)("li",{parentName:"ul"},"Detectors: ",(0,i.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout/tree/main/detectors/avoid-format!-string"},(0,i.kt)("inlineCode",{parentName:"a"},"avoid-format!-string"))),(0,i.kt)("li",{parentName:"ul"},"Test Cases: ",(0,i.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout/tree/main/test-cases/avoid-format!-string/avoid-format!-string-1"},(0,i.kt)("inlineCode",{parentName:"a"},"avoid-format!-string-1")))),(0,i.kt)("p",null,"The ",(0,i.kt)("inlineCode",{parentName:"p"},"format!")," macro is not recommended. A custom error is recommended instead."),(0,i.kt)("h2",{id:"exploit-scenario"},"Exploit Scenario"),(0,i.kt)("p",null,"Consider the following ",(0,i.kt)("inlineCode",{parentName:"p"},"ink!")," contract:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},'    #[ink(message)]\n    pub fn crash(&self) -> Result<(), Error> {\n        Err(Error::FormatError {\n            msg: (format!("{:?}", "false")),\n        })\n    }\n')),(0,i.kt)("p",null,"The problem arises from the use of the ",(0,i.kt)("inlineCode",{parentName:"p"},"format!")," macro. This is used to format a string with the given arguments. This is a bad practice because it can panic the execution."),(0,i.kt)("p",null,"The vulnerable code example can be found ",(0,i.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout/tree/main/test-cases/avoid-format!-string/avoid-format!-string-1/vulnerable-example"},(0,i.kt)("inlineCode",{parentName:"a"},"here")),"."),(0,i.kt)("h2",{id:"remediation"},"Remediation"),(0,i.kt)("p",null,"Create a custom error to avoid using the macro."),(0,i.kt)("h2",{id:"references"},"References"),(0,i.kt)("ul",null,(0,i.kt)("li",{parentName:"ul"},(0,i.kt)("a",{parentName:"li",href:"https://docs.alephzero.org/aleph-zero/security-course-by-kudelski-security/ink-developers-security-guideline#be-careful-when-you-use-the-following-patterns-that-may-cause-panics."},"Memory Management"))))}p.isMDXComponent=!0}}]);