"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[8280],{9613:(e,t,n)=>{n.d(t,{Zo:()=>p,kt:()=>f});var r=n(9496);function o(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function a(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?a(Object(n),!0).forEach((function(t){o(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):a(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function s(e,t){if(null==e)return{};var n,r,o=function(e,t){if(null==e)return{};var n,r,o={},a=Object.keys(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||(o[n]=e[n]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(o[n]=e[n])}return o}var u=r.createContext({}),l=function(e){var t=r.useContext(u),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},p=function(e){var t=l(e.components);return r.createElement(u.Provider,{value:t},e.children)},c="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},m=r.forwardRef((function(e,t){var n=e.components,o=e.mdxType,a=e.originalType,u=e.parentName,p=s(e,["components","mdxType","originalType","parentName"]),c=l(n),m=o,f=c["".concat(u,".").concat(m)]||c[m]||d[m]||a;return n?r.createElement(f,i(i({ref:t},p),{},{components:n})):r.createElement(f,i({ref:t},p))}));function f(e,t){var n=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=n.length,i=new Array(a);i[0]=m;var s={};for(var u in t)hasOwnProperty.call(t,u)&&(s[u]=t[u]);s.originalType=e,s[c]="string"==typeof e?e:o,i[1]=s;for(var l=2;l<a;l++)i[l]=n[l];return r.createElement.apply(null,i)}return r.createElement.apply(null,n)}m.displayName="MDXCreateElement"},5326:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>u,contentTitle:()=>i,default:()=>d,frontMatter:()=>a,metadata:()=>s,toc:()=>l});var r=n(2564),o=(n(9496),n(9613));const a={},i="DoS unbounded operation",s={unversionedId:"detectors/dos-unbounded-operation",id:"detectors/dos-unbounded-operation",title:"DoS unbounded operation",description:"What it does",source:"@site/docs/detectors/6-dos-unbounded-operation.md",sourceDirName:"detectors",slug:"/detectors/dos-unbounded-operation",permalink:"/scout/docs/detectors/dos-unbounded-operation",draft:!1,editUrl:"https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/docs/detectors/6-dos-unbounded-operation.md",tags:[],version:"current",sidebarPosition:6,frontMatter:{},sidebar:"docsSidebar",previous:{title:"Unused return enum",permalink:"/scout/docs/detectors/unused-return-enum"},next:{title:"DoS unexpected revert with vector",permalink:"/scout/docs/detectors/dos-unexpected-revert-with-vector"}},u={},l=[{value:"What it does",id:"what-it-does",level:3},{value:"Why is this bad?",id:"why-is-this-bad",level:3},{value:"Known problems",id:"known-problems",level:3},{value:"Example",id:"example",level:3},{value:"Implementation",id:"implementation",level:3}],p={toc:l},c="wrapper";function d(e){let{components:t,...n}=e;return(0,o.kt)(c,(0,r.Z)({},p,n,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("h1",{id:"dos-unbounded-operation"},"DoS unbounded operation"),(0,o.kt)("h3",{id:"what-it-does"},"What it does"),(0,o.kt)("p",null,"This detector checks that when using for or while loops, their conditions limit the execution to a constant number of iterations."),(0,o.kt)("h3",{id:"why-is-this-bad"},"Why is this bad?"),(0,o.kt)("p",null,"If the number of iterations is not limited to a specific range, it could potentially cause out of gas exceptions."),(0,o.kt)("h3",{id:"known-problems"},"Known problems"),(0,o.kt)("p",null,"False positives are to be expected when using variables that can only be set using controlled flows that limit the values within acceptable ranges."),(0,o.kt)("h3",{id:"example"},"Example"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},"pub fn pay_out(&mut self) {\n    for i in 0..self.next_payee_ix {\n        let payee = self.payees.get(&i).unwrap();\n        self.env().transfer(payee.address, payee.value).unwrap();\n    }\n}\n")),(0,o.kt)("p",null,"Use instead:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},"pub fn pay_out(&mut self, payee: u128) {\n    let payee = self.payees.get(&payee).unwrap();\n    self.env().transfer(payee.address, payee.value).unwrap();\n}\n")),(0,o.kt)("h3",{id:"implementation"},"Implementation"),(0,o.kt)("p",null,"The detector's implementation can be found at ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout/tree/main/detectors/dos-unbounded-operation"},"this link"),"."))}d.isMDXComponent=!0}}]);