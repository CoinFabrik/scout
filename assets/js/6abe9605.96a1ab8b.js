"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[3372],{9613:(e,t,n)=>{n.d(t,{Zo:()=>u,kt:()=>g});var a=n(9496);function l(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function r(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?r(Object(n),!0).forEach((function(t){l(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):r(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function o(e,t){if(null==e)return{};var n,a,l=function(e,t){if(null==e)return{};var n,a,l={},r=Object.keys(e);for(a=0;a<r.length;a++)n=r[a],t.indexOf(n)>=0||(l[n]=e[n]);return l}(e,t);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);for(a=0;a<r.length;a++)n=r[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(l[n]=e[n])}return l}var c=a.createContext({}),s=function(e){var t=a.useContext(c),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},u=function(e){var t=s(e.components);return a.createElement(c.Provider,{value:t},e.children)},p="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},m=a.forwardRef((function(e,t){var n=e.components,l=e.mdxType,r=e.originalType,c=e.parentName,u=o(e,["components","mdxType","originalType","parentName"]),p=s(n),m=l,g=p["".concat(c,".").concat(m)]||p[m]||d[m]||r;return n?a.createElement(g,i(i({ref:t},u),{},{components:n})):a.createElement(g,i({ref:t},u))}));function g(e,t){var n=arguments,l=t&&t.mdxType;if("string"==typeof e||l){var r=n.length,i=new Array(r);i[0]=m;var o={};for(var c in t)hasOwnProperty.call(t,c)&&(o[c]=t[c]);o.originalType=e,o[p]="string"==typeof e?e:l,i[1]=o;for(var s=2;s<r;s++)i[s]=n[s];return a.createElement.apply(null,i)}return a.createElement.apply(null,n)}m.displayName="MDXCreateElement"},234:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>c,contentTitle:()=>i,default:()=>d,frontMatter:()=>r,metadata:()=>o,toc:()=>s});var a=n(2564),l=(n(9496),n(9613));const r={},i="Delegate Call",o={unversionedId:"vulnerabilities/delegate-call",id:"vulnerabilities/delegate-call",title:"Delegate Call",description:"Description",source:"@site/docs/vulnerabilities/11-delegate-call.md",sourceDirName:"vulnerabilities",slug:"/vulnerabilities/delegate-call",permalink:"/scout/docs/vulnerabilities/delegate-call",draft:!1,editUrl:"https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/docs/vulnerabilities/11-delegate-call.md",tags:[],version:"current",sidebarPosition:11,frontMatter:{},sidebar:"docsSidebar",previous:{title:"Divide before multiply",permalink:"/scout/docs/vulnerabilities/divide-before-multiply"},next:{title:"Detectors",permalink:"/scout/docs/detectors/"}},c={},s=[{value:"Description",id:"description",level:2},{value:"Exploit Scenario",id:"exploit-scenario",level:2},{value:"Remediation",id:"remediation",level:2},{value:"Reference",id:"reference",level:2}],u={toc:s},p="wrapper";function d(e){let{components:t,...n}=e;return(0,l.kt)(p,(0,a.Z)({},u,n,{components:t,mdxType:"MDXLayout"}),(0,l.kt)("h1",{id:"delegate-call"},"Delegate Call"),(0,l.kt)("h2",{id:"description"},"Description"),(0,l.kt)("ul",null,(0,l.kt)("li",{parentName:"ul"},"Vulnerability Category: ",(0,l.kt)("inlineCode",{parentName:"li"},"Unsecure delegate calls")),(0,l.kt)("li",{parentName:"ul"},"Vulnerability Severity: ",(0,l.kt)("inlineCode",{parentName:"li"},"Major")),(0,l.kt)("li",{parentName:"ul"},"Detectors: ",(0,l.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout/tree/main/detectors/delegate-call"},(0,l.kt)("inlineCode",{parentName:"a"},"delegate-call"))),(0,l.kt)("li",{parentName:"ul"},"Test Cases: ",(0,l.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout/tree/main/test-cases/delegate-call/delegate-call-1"},(0,l.kt)("inlineCode",{parentName:"a"},"delegate-call-1")))),(0,l.kt)("p",null,"In Rust, the ",(0,l.kt)("inlineCode",{parentName:"p"},"delegate call")," is used to invoke a method from another contract. If the target of the delegate call is passed as an argument, it can be used to change the expected behavior of the contract. This can be exploited maliciously to disrupt the contract's operation."),(0,l.kt)("h2",{id:"exploit-scenario"},"Exploit Scenario"),(0,l.kt)("p",null,"Consider the following ",(0,l.kt)("inlineCode",{parentName:"p"},"ink!")," contract:"),(0,l.kt)("pre",null,(0,l.kt)("code",{parentName:"pre",className:"language-rust"},"#[ink::contract]\nmod delegate_call {\n\n    use ink::env::{\n        call::{build_call, ExecutionInput, Selector},\n        DefaultEnvironment,\n    };\n\n    #[ink(storage)]\n    pub struct DelegateCall {\n        admin: AccountId,\n        addresses: [AccountId; 3],\n        percent1: u128,\n        percent2: u128,\n        percent3: u128,\n    }\n\n    impl DelegateCall {\n\n        // ...\n\n        /// Delegates the fee calculation and pays the results to the corresponding addresses\n        #[ink(message, payable)]\n        pub fn ask_payouts(&mut self, target: Hash) -> Result<(Balance, Balance, Balance), Error> {\n            let amount = self.env().transferred_value();\n\n            let result: (Balance, Balance, Balance) = build_call::<DefaultEnvironment>()\n                .delegate(target)\n                // ...\n        }\n    }\n}\n")),(0,l.kt)("p",null,"In this contract, the ",(0,l.kt)("inlineCode",{parentName:"p"},"ask_payouts")," function takes a ",(0,l.kt)("inlineCode",{parentName:"p"},"Hash")," as a target and delegates a call to that target. A malicious user could potentially manipulate the function to their advantage by providing a malicious ",(0,l.kt)("inlineCode",{parentName:"p"},"Hash")," as the target."),(0,l.kt)("p",null,"The vulnerable code example can be found ",(0,l.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout/tree/main/test-cases/delegate-call/delegate-call-1/vulnerable-example"},(0,l.kt)("inlineCode",{parentName:"a"},"here")),"."),(0,l.kt)("h2",{id:"remediation"},"Remediation"),(0,l.kt)("p",null,"Instead of passing the target of a delegate call as an argument, use a storage variable (like ",(0,l.kt)("inlineCode",{parentName:"p"},"self.target"),"). Also, provide a function with proper access control to change the target."),(0,l.kt)("pre",null,(0,l.kt)("code",{parentName:"pre",className:"language-rust"},'#[ink::contract]\nmod delegate_call {\n\n    #[ink(storage)]\n    pub struct DelegateCall {\n        admin: AccountId,\n        addresses: [AccountId; 3],\n        target: Hash,\n    }\n\n    impl DelegateCall {\n\n        // ...\n\n        /// Delegates the fee calculation and pays the results to the corresponding addresses\n        #[ink(message, payable)]\n        pub fn ask_payouts(&mut self, amount: Balance) -> Result<(), Error> {\n            let result = ink::env::call::build_call::<ink::env::DefaultEnvironment>()\n                .delegate(self.target)\n                // ...\n        }\n\n        /// Sets the target codehash for the delegated call\n        #[ink(message)]\n        pub fn set_target(&mut self, new_target: Hash) -> Result<(), &\'static str> {\n           if self.admin != self.env().caller() {\n                Err("Only admin can set target")\n            } else {\n                self.target = new_target;\n                Ok(())\n            }\n        }\n\n    }\n}\n')),(0,l.kt)("p",null,"The remediated code example can be found ",(0,l.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout/tree/main/test-cases/delegate-call/delegate-call-1/remediated-example"},(0,l.kt)("inlineCode",{parentName:"a"},"here")),"."),(0,l.kt)("h2",{id:"reference"},"Reference"),(0,l.kt)("p",null,(0,l.kt)("a",{parentName:"p",href:"https://paritytech.github.io/ink/ink_env/call/struct.DelegateCall.html"},"Ink! documentation: DelegateCall")))}d.isMDXComponent=!0}}]);