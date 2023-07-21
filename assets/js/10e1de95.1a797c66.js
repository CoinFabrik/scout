"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[7139],{9613:(e,t,n)=>{n.d(t,{Zo:()=>u,kt:()=>m});var r=n(9496);function o(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function a(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function s(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?a(Object(n),!0).forEach((function(t){o(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):a(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,r,o=function(e,t){if(null==e)return{};var n,r,o={},a=Object.keys(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||(o[n]=e[n]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(o[n]=e[n])}return o}var i=r.createContext({}),c=function(e){var t=r.useContext(i),n=t;return e&&(n="function"==typeof e?e(t):s(s({},t),e)),n},u=function(e){var t=c(e.components);return r.createElement(i.Provider,{value:t},e.children)},p="mdxType",f={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},d=r.forwardRef((function(e,t){var n=e.components,o=e.mdxType,a=e.originalType,i=e.parentName,u=l(e,["components","mdxType","originalType","parentName"]),p=c(n),d=o,m=p["".concat(i,".").concat(d)]||p[d]||f[d]||a;return n?r.createElement(m,s(s({ref:t},u),{},{components:n})):r.createElement(m,s({ref:t},u))}));function m(e,t){var n=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=n.length,s=new Array(a);s[0]=d;var l={};for(var i in t)hasOwnProperty.call(t,i)&&(l[i]=t[i]);l.originalType=e,l[p]="string"==typeof e?e:o,s[1]=l;for(var c=2;c<a;c++)s[c]=n[c];return r.createElement.apply(null,s)}return r.createElement.apply(null,n)}d.displayName="MDXCreateElement"},9065:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>i,contentTitle:()=>s,default:()=>f,frontMatter:()=>a,metadata:()=>l,toc:()=>c});var r=n(2564),o=(n(9496),n(9613));const a={},s="Unrestricted Transfer From",l={unversionedId:"detectors/unrestricted-transfer-from",id:"detectors/unrestricted-transfer-from",title:"Unrestricted Transfer From",description:"What it does",source:"@site/docs/detectors/14-unrestricted-transfer-from.md",sourceDirName:"detectors",slug:"/detectors/unrestricted-transfer-from",permalink:"/scout/docs/detectors/unrestricted-transfer-from",draft:!1,editUrl:"https://github.com/CoinFabrik/scout/docs/detectors/14-unrestricted-transfer-from.md",tags:[],version:"current",sidebarPosition:14,frontMatter:{},sidebar:"docsSidebar",previous:{title:"Insuficciently random values",permalink:"/scout/docs/detectors/insufficiently-random-values"},next:{title:"Assert violation",permalink:"/scout/docs/detectors/assert-violation"}},i={},c=[{value:"What it does",id:"what-it-does",level:3},{value:"Why is this bad?",id:"why-is-this-bad",level:3},{value:"Known problems",id:"known-problems",level:3},{value:"Example",id:"example",level:3},{value:"Implementation",id:"implementation",level:3}],u={toc:c},p="wrapper";function f(e){let{components:t,...n}=e;return(0,o.kt)(p,(0,r.Z)({},u,n,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("h1",{id:"unrestricted-transfer-from"},"Unrestricted Transfer From"),(0,o.kt)("h3",{id:"what-it-does"},"What it does"),(0,o.kt)("p",null,"It warns you if a ",(0,o.kt)("inlineCode",{parentName:"p"},"transfer_from")," function is called with a user-defined parameter in the ",(0,o.kt)("inlineCode",{parentName:"p"},"from")," field."),(0,o.kt)("h3",{id:"why-is-this-bad"},"Why is this bad?"),(0,o.kt)("p",null,"An user Alice can approve a contract to spend their tokens. An user Bob can call that contract, use that allowance to send themselves Alice's tokens. "),(0,o.kt)("h3",{id:"known-problems"},"Known problems"),(0,o.kt)("p",null,"None."),(0,o.kt)("h3",{id:"example"},"Example"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},'// build_call example\n    #[ink(message)]\n    pub fn deposit(&mut self, from: AccountId) -> Result<(), Error> {\n        let call_params = build_call::<DefaultEnvironment>()\n            .exec_input(\n                ExecutionInput::new(Selector::new(ink::selector_bytes!(\n                    "PSP22::transfer_from"\n                )))\n                .push_arg(from)\n                .push_arg(self.env().account_id())\n                .push_arg(self.amount)\n                .push_arg([0u8]),\n            )\n    }\n// ContractRef example\n    #[ink(message)]\n    pub fn deposit(&mut self, from: AccountId) -> Result<(), Error> {\n        let res = PSP22Ref::transfer_from(\n            &self.psp22_address,\n            from,\n            self.env().account_id(),\n            self.amount,\n            vec![],\n        );\n    }\n')),(0,o.kt)("p",null,"Use instead:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},'// build_call example\n    pub fn deposit(&mut self) -> Result<(), Error> {\n        let call_params = build_call::<DefaultEnvironment>()\n            .exec_input(\n                ExecutionInput::new(Selector::new(ink::selector_bytes!(\n                    "PSP22::transfer_from"\n                )))\n                .push_arg(self.env().caller())\n                .push_arg(self.env().account_id())\n                .push_arg(self.amount)\n                .push_arg([0u8]),\n            )\n    }\n\n// ContractRef example\n    pub fn deposit(&mut self) -> Result<(), Error> {\n        let res = PSP22Ref::transfer_from(\n            &self.psp22_address,\n            self.env().caller(),\n            self.env().account_id(),\n            self.amount,\n            vec![],\n        );\n    }\n\n')),(0,o.kt)("h3",{id:"implementation"},"Implementation"),(0,o.kt)("p",null,"The detector's implementation can be found at ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout/tree/main/detectors/unrestricted-transfer-from"},"this link"),"."))}f.isMDXComponent=!0}}]);