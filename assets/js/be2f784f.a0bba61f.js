"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[9152],{9613:(e,t,r)=>{r.d(t,{Zo:()=>c,kt:()=>f});var n=r(9496);function a(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function i(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function o(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?i(Object(r),!0).forEach((function(t){a(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):i(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function l(e,t){if(null==e)return{};var r,n,a=function(e,t){if(null==e)return{};var r,n,a={},i=Object.keys(e);for(n=0;n<i.length;n++)r=i[n],t.indexOf(r)>=0||(a[r]=e[r]);return a}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(n=0;n<i.length;n++)r=i[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(a[r]=e[r])}return a}var s=n.createContext({}),p=function(e){var t=n.useContext(s),r=t;return e&&(r="function"==typeof e?e(t):o(o({},t),e)),r},c=function(e){var t=p(e.components);return n.createElement(s.Provider,{value:t},e.children)},u="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},m=n.forwardRef((function(e,t){var r=e.components,a=e.mdxType,i=e.originalType,s=e.parentName,c=l(e,["components","mdxType","originalType","parentName"]),u=p(r),m=a,f=u["".concat(s,".").concat(m)]||u[m]||d[m]||i;return r?n.createElement(f,o(o({ref:t},c),{},{components:r})):n.createElement(f,o({ref:t},c))}));function f(e,t){var r=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var i=r.length,o=new Array(i);o[0]=m;var l={};for(var s in t)hasOwnProperty.call(t,s)&&(l[s]=t[s]);l.originalType=e,l[u]="string"==typeof e?e:a,o[1]=l;for(var p=2;p<i;p++)o[p]=r[p];return n.createElement.apply(null,o)}return n.createElement.apply(null,r)}m.displayName="MDXCreateElement"},4428:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>s,contentTitle:()=>o,default:()=>d,frontMatter:()=>i,metadata:()=>l,toc:()=>p});var n=r(2564),a=(r(9496),r(9613));const i={},o="Unrestricted Transfer From",l={unversionedId:"vulnerabilities/unprotected-mapping-operation",id:"vulnerabilities/unprotected-mapping-operation",title:"Unrestricted Transfer From",description:"Description",source:"@site/docs/vulnerabilities/22-unprotected-mapping-operation.md",sourceDirName:"vulnerabilities",slug:"/vulnerabilities/unprotected-mapping-operation",permalink:"/scout/docs/vulnerabilities/unprotected-mapping-operation",draft:!1,editUrl:"https://github.com/CoinFabrik/scout/docs/vulnerabilities/22-unprotected-mapping-operation.md",tags:[],version:"current",sidebarPosition:22,frontMatter:{},sidebar:"docsSidebar",previous:{title:"Unprotected Set Code Hash",permalink:"/scout/docs/vulnerabilities/unprotected-set-code-hash"},next:{title:"Detectors",permalink:"/scout/docs/detectors/"}},s={},p=[{value:"Description",id:"description",level:2},{value:"Exploit Scenario",id:"exploit-scenario",level:2},{value:"Remediation",id:"remediation",level:2},{value:"References",id:"references",level:2}],c={toc:p},u="wrapper";function d(e){let{components:t,...r}=e;return(0,a.kt)(u,(0,n.Z)({},c,r,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h1",{id:"unrestricted-transfer-from"},"Unrestricted Transfer From"),(0,a.kt)("h2",{id:"description"},"Description"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},"Vulnerability Category: ",(0,a.kt)("inlineCode",{parentName:"li"},"Authorization")),(0,a.kt)("li",{parentName:"ul"},"Vulnerability Severity: ",(0,a.kt)("inlineCode",{parentName:"li"},"Critical")),(0,a.kt)("li",{parentName:"ul"},"Detectors: ",(0,a.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout/tree/main/detectors/unprotected-mapping-operation"},(0,a.kt)("inlineCode",{parentName:"a"},"unprotected-mapping-operation"))),(0,a.kt)("li",{parentName:"ul"},"Test Cases: ",(0,a.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout/tree/main/test-cases/unprotected-mapping-operation/unprotected-mapping-operation1"},(0,a.kt)("inlineCode",{parentName:"a"},"unprotected-mapping-operation-1")))),(0,a.kt)("p",null,"Modifying mappings with an arbitrary key given by users can be a significant vulnerability for several reasons:"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("p",{parentName:"li"},"Unintended Modifications: Allowing users to provide arbitrary keys can lead to unintended modifications of critical data within the smart contract. If the input validation and sanitation are not done properly, users may be able to manipulate the data in ways that were not intended by the contract's author.")),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("p",{parentName:"li"},"Data Corruption: Malicious users could intentionally provide keys that result in the corruption or manipulation of important data stored in the mapping. This could lead to incorrect calculations, unauthorized access, or other undesirable outcomes.")),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("p",{parentName:"li"},"Denial-of-Service (DoS) Attacks: If users can set arbitrary keys, they may be able to create mappings with a large number of entries, potentially causing the contract to exceed its gas limit. This could lead to denial-of-service attacks, making the contract unusable for other users."))),(0,a.kt)("h2",{id:"exploit-scenario"},"Exploit Scenario"),(0,a.kt)("p",null,"Consider the following ",(0,a.kt)("inlineCode",{parentName:"p"},"ink!")," contract:"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"    #[ink(message)]\n    pub fn withdraw(&mut self, amount: Balance, from: AccountId) -> Result<(), Error> {\n        let current_bal = self.balances.take(from).unwrap_or(0);\n        if current_bal >= amount {\n            self.balances.insert(from, &(current_bal - amount));\n            self.env()\n                .transfer(from, current_bal)\n                .map_err(|_| Error::TransferError)\n        } else {\n            Err(Error::BalanceNotEnough)\n        }\n    }\n")),(0,a.kt)("p",null,"The vulnerability in this ",(0,a.kt)("inlineCode",{parentName:"p"},"withdraw")," function arises from the use of ",(0,a.kt)("inlineCode",{parentName:"p"},"from"),", an user-defined parameter used as key in the mapping without prior sanitizing. Alice can withdraw tokens from any user to the user balance. "),(0,a.kt)("p",null,"The vulnerable code example can be found ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout/tree/main/test-cases/unprotected-mapping-operation/unprotected-mapping-operation1/vulnerable-example"},(0,a.kt)("inlineCode",{parentName:"a"},"here")),"."),(0,a.kt)("h2",{id:"remediation"},"Remediation"),(0,a.kt)("p",null,"Avoid using user-given arguments as ",(0,a.kt)("inlineCode",{parentName:"p"},"key")," parameter in mapping. Instead, use ",(0,a.kt)("inlineCode",{parentName:"p"},"self.env().caller()")," or sanitize the values."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"    #[ink(message)]\n    pub fn withdraw(&mut self, amount: Balance) -> Result<(), Error> {\n        let caller = self.env().caller();\n        let current_bal = self.balances.take(caller).unwrap_or(0);\n        if current_bal >= amount {\n            self.balances.insert(caller, &(current_bal - amount));\n            self.env()\n                .transfer(caller, current_bal)\n                .map_err(|_| Error::TransferError)\n        } else {\n            Err(Error::BalanceNotEnough)\n        }\n    }\n")),(0,a.kt)("h2",{id:"references"},"References"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"https://docs.alephzero.org/aleph-zero/security-course-by-kudelski-security/ink-developers-security-guideline#unprotected-self-destruction-or-burning-instruction-s"},"Aleph Zero ink! Developer security guidelines"))))}d.isMDXComponent=!0}}]);