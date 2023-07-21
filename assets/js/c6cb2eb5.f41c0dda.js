"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[1884],{9613:(e,t,r)=>{r.d(t,{Zo:()=>u,kt:()=>d});var n=r(9496);function a(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function i(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function o(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?i(Object(r),!0).forEach((function(t){a(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):i(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function s(e,t){if(null==e)return{};var r,n,a=function(e,t){if(null==e)return{};var r,n,a={},i=Object.keys(e);for(n=0;n<i.length;n++)r=i[n],t.indexOf(r)>=0||(a[r]=e[r]);return a}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(n=0;n<i.length;n++)r=i[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(a[r]=e[r])}return a}var l=n.createContext({}),c=function(e){var t=n.useContext(l),r=t;return e&&(r="function"==typeof e?e(t):o(o({},t),e)),r},u=function(e){var t=c(e.components);return n.createElement(l.Provider,{value:t},e.children)},p="mdxType",f={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},m=n.forwardRef((function(e,t){var r=e.components,a=e.mdxType,i=e.originalType,l=e.parentName,u=s(e,["components","mdxType","originalType","parentName"]),p=c(r),m=a,d=p["".concat(l,".").concat(m)]||p[m]||f[m]||i;return r?n.createElement(d,o(o({ref:t},u),{},{components:r})):n.createElement(d,o({ref:t},u))}));function d(e,t){var r=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var i=r.length,o=new Array(i);o[0]=m;var s={};for(var l in t)hasOwnProperty.call(t,l)&&(s[l]=t[l]);s.originalType=e,s[p]="string"==typeof e?e:a,o[1]=s;for(var c=2;c<i;c++)o[c]=r[c];return n.createElement.apply(null,o)}return n.createElement.apply(null,r)}m.displayName="MDXCreateElement"},4893:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>l,contentTitle:()=>o,default:()=>f,frontMatter:()=>i,metadata:()=>s,toc:()=>c});var n=r(2564),a=(r(9496),r(9613));const i={},o="Unrestricted Transfer From",s={unversionedId:"vulnerabilities/unrestricted-transfer-from",id:"vulnerabilities/unrestricted-transfer-from",title:"Unrestricted Transfer From",description:"Description",source:"@site/docs/vulnerabilities/14-unrestricted-transfer-from.md",sourceDirName:"vulnerabilities",slug:"/vulnerabilities/unrestricted-transfer-from",permalink:"/scout/docs/vulnerabilities/unrestricted-transfer-from",draft:!1,editUrl:"https://github.com/CoinFabrik/scout/docs/vulnerabilities/14-unrestricted-transfer-from.md",tags:[],version:"current",sidebarPosition:14,frontMatter:{},sidebar:"docsSidebar",previous:{title:"Insufficiently random values",permalink:"/scout/docs/vulnerabilities/insufficiently-random-values"},next:{title:"Assert violation",permalink:"/scout/docs/vulnerabilities/assert-violation"}},l={},c=[{value:"Description",id:"description",level:2},{value:"Exploit Scenario",id:"exploit-scenario",level:2},{value:"Remediation",id:"remediation",level:2},{value:"References",id:"references",level:2}],u={toc:c},p="wrapper";function f(e){let{components:t,...r}=e;return(0,a.kt)(p,(0,n.Z)({},u,r,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h1",{id:"unrestricted-transfer-from"},"Unrestricted Transfer From"),(0,a.kt)("h2",{id:"description"},"Description"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},"Vulnerability Category: ",(0,a.kt)("inlineCode",{parentName:"li"},"Validations and error handling")),(0,a.kt)("li",{parentName:"ul"},"Vulnerability Severity: ",(0,a.kt)("inlineCode",{parentName:"li"},"High")),(0,a.kt)("li",{parentName:"ul"},"Detectors: ",(0,a.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout/tree/main/detectors/unrestricted-transfer-from"},(0,a.kt)("inlineCode",{parentName:"a"},"unrestricted-transfer-from"))),(0,a.kt)("li",{parentName:"ul"},"Test Cases: ",(0,a.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout/tree/main/test-cases/unrestricted-transfer-from/unrestricted-transfer-from-1"},(0,a.kt)("inlineCode",{parentName:"a"},"unrestricted-transfer-from-1")))),(0,a.kt)("p",null,"Using an user-defined argument as a ",(0,a.kt)("inlineCode",{parentName:"p"},"transfer_from"),"'s ",(0,a.kt)("inlineCode",{parentName:"p"},"from")," parameter could lead to transfer funds from a third party account without proper authorization."),(0,a.kt)("h2",{id:"exploit-scenario"},"Exploit Scenario"),(0,a.kt)("p",null,"Consider the following ",(0,a.kt)("inlineCode",{parentName:"p"},"ink!")," contract:"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},'// build_call example\n    #[ink(message)]\n    pub fn deposit(&mut self, from: AccountId) -> Result<(), Error> {\n        let call_params = build_call::<DefaultEnvironment>()\n            .exec_input(\n                ExecutionInput::new(Selector::new(ink::selector_bytes!(\n                    "PSP22::transfer_from"\n                )))\n                .push_arg(from)\n                .push_arg(self.env().account_id())\n                .push_arg(self.amount)\n                .push_arg([0u8]),\n            )\n    }\n\n// ContractRef example\n    #[ink(message)]\n    pub fn deposit(&mut self, from: AccountId) -> Result<(), Error> {\n        let res = PSP22Ref::transfer_from(\n            &self.psp22_address,\n            from,\n            self.env().account_id(),\n            self.amount,\n            vec![],\n        );\n    }\n')),(0,a.kt)("p",null,"The vulnerability in this ",(0,a.kt)("inlineCode",{parentName:"p"},"deposit")," function arises from the use of ",(0,a.kt)("inlineCode",{parentName:"p"},"from"),", an user-defined parameter as an argument in the ",(0,a.kt)("inlineCode",{parentName:"p"},"from")," field of the ",(0,a.kt)("inlineCode",{parentName:"p"},"transfer_from")," function. Alice can approve a contract to spend their tokens, then Bob can call that contract, use that allowance to send as themselves Alice's tokens."),(0,a.kt)("p",null,"The vulnerable code example can be found ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout/tree/main/test-cases/unrestricted-transfer-from/unrestricted-transfer-from-1/vulnerable-example"},(0,a.kt)("inlineCode",{parentName:"a"},"here")),"."),(0,a.kt)("h2",{id:"remediation"},"Remediation"),(0,a.kt)("p",null,"Avoid using user-defined arguments as ",(0,a.kt)("inlineCode",{parentName:"p"},"from")," parameter in ",(0,a.kt)("inlineCode",{parentName:"p"},"transfer_from"),". Instead, use ",(0,a.kt)("inlineCode",{parentName:"p"},"self.env().caller()"),"."),(0,a.kt)("p",null,"The remediated code example can be found ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout/tree/main/test-cases/unrestricted-transfer-from/unrestricted-transfer-from-1/remediated-example"},(0,a.kt)("inlineCode",{parentName:"a"},"here")),"."),(0,a.kt)("h2",{id:"references"},"References"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"https://github.com/crytic/slither/wiki/Detector-Documentation#arbitrary-from-in-transferfrom"},"Slither: Arbitrary from in transferFrom"))))}f.isMDXComponent=!0}}]);