"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[5136],{9613:(e,t,n)=>{n.d(t,{Zo:()=>u,kt:()=>f});var r=n(9496);function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function l(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function o(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?l(Object(n),!0).forEach((function(t){a(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):l(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function c(e,t){if(null==e)return{};var n,r,a=function(e,t){if(null==e)return{};var n,r,a={},l=Object.keys(e);for(r=0;r<l.length;r++)n=l[r],t.indexOf(n)>=0||(a[n]=e[n]);return a}(e,t);if(Object.getOwnPropertySymbols){var l=Object.getOwnPropertySymbols(e);for(r=0;r<l.length;r++)n=l[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(a[n]=e[n])}return a}var i=r.createContext({}),s=function(e){var t=r.useContext(i),n=t;return e&&(n="function"==typeof e?e(t):o(o({},t),e)),n},u=function(e){var t=s(e.components);return r.createElement(i.Provider,{value:t},e.children)},d="mdxType",p={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},m=r.forwardRef((function(e,t){var n=e.components,a=e.mdxType,l=e.originalType,i=e.parentName,u=c(e,["components","mdxType","originalType","parentName"]),d=s(n),m=a,f=d["".concat(i,".").concat(m)]||d[m]||p[m]||l;return n?r.createElement(f,o(o({ref:t},u),{},{components:n})):r.createElement(f,o({ref:t},u))}));function f(e,t){var n=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var l=n.length,o=new Array(l);o[0]=m;var c={};for(var i in t)hasOwnProperty.call(t,i)&&(c[i]=t[i]);c.originalType=e,c[d]="string"==typeof e?e:a,o[1]=c;for(var s=2;s<l;s++)o[s]=n[s];return r.createElement.apply(null,o)}return r.createElement.apply(null,n)}m.displayName="MDXCreateElement"},5715:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>i,contentTitle:()=>o,default:()=>p,frontMatter:()=>l,metadata:()=>c,toc:()=>s});var r=n(2564),a=(n(9496),n(9613));const l={},o="Reentrancy",c={unversionedId:"detectors/reentrancy",id:"detectors/reentrancy",title:"Reentrancy",description:"What it does",source:"@site/docs/detectors/3-reentrancy.md",sourceDirName:"detectors",slug:"/detectors/reentrancy",permalink:"/scout/docs/detectors/reentrancy",draft:!1,editUrl:"https://github.com/CoinFabrik/scout/docs/detectors/3-reentrancy.md",tags:[],version:"current",sidebarPosition:3,frontMatter:{},sidebar:"docsSidebar",previous:{title:"Set contract storage",permalink:"/scout/docs/detectors/set-contract-storage"},next:{title:"Panic error",permalink:"/scout/docs/detectors/panic-error"}},i={},s=[{value:"What it does",id:"what-it-does",level:3},{value:"Why is this bad?",id:"why-is-this-bad",level:3},{value:"Known problems",id:"known-problems",level:3},{value:"Example",id:"example",level:3},{value:"Implementation",id:"implementation",level:3}],u={toc:s},d="wrapper";function p(e){let{components:t,...n}=e;return(0,a.kt)(d,(0,r.Z)({},u,n,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h1",{id:"reentrancy"},"Reentrancy"),(0,a.kt)("h3",{id:"what-it-does"},"What it does"),(0,a.kt)("p",null,"This linting rule checks whether the 'check-effect' interaction pattern has been properly followed by code that invokes a contract that may call back the original one."),(0,a.kt)("h3",{id:"why-is-this-bad"},"Why is this bad?"),(0,a.kt)("p",null,"If state modifications are made after a contract call, reentrant calls may not detect these modifications, potentially leading to unexpected behaviors such as double spending."),(0,a.kt)("h3",{id:"known-problems"},"Known problems"),(0,a.kt)("p",null,"If called method does not perform a malicious reentrancy (i.e. known method from known contract) false positives will arise.\nIf the usage of set_allow_reentry(true) or later state changes are performed in an auxiliary function, this detector will not detect the reentrancy."),(0,a.kt)("h3",{id:"example"},"Example"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"let caller_addr = self.env().caller();\nlet caller_balance = self.balance(caller_addr);\n\nif amount > caller_balance {\n    return Ok(caller_balance);\n}\n\nlet call = build_call::<ink::env::DefaultEnvironment>()\n    .call(address)\n    .transferred_value(amount)\n    .exec_input(ink::env::call::ExecutionInput::new(Selector::new(\n        selector.to_be_bytes(),\n    )))\n    .call_flags(ink::env::CallFlags::default().set_allow_reentry(true))\n    .returns::<()>()\n    .params();\nself.env()\n    .invoke_contract(&call)\n    .map_err(|_| Error::ContractInvokeFailed)?\n    .map_err(|_| Error::ContractInvokeFailed)?;\n\nlet new_balance = caller_balance.checked_sub(amount).ok_or(Error::Underflow)?;\nself.balances.insert(caller_addr, &new_balance);\n")),(0,a.kt)("p",null,"Use instead:"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},'let caller_addr = self.env().caller();\nlet caller_balance = self.balances.get(caller_addr).unwrap_or(0);\nif amount <= caller_balance {\n    //The balance is updated before the contract call\n    self.balances\n        .insert(caller_addr, &(caller_balance - amount));\n    let call = build_call::<ink::env::DefaultEnvironment>()\n        .call(address)\n        .transferred_value(amount)\n        .exec_input(ink::env::call::ExecutionInput::new(Selector::new(\n            selector.to_be_bytes(),\n        )))\n        .call_flags(ink::env::CallFlags::default().set_allow_reentry(true))\n        .returns::<()>()\n        .params();\n    self.env()\n        .invoke_contract(&call)\n        .unwrap_or_else(|err| panic!("Err {:?}", err))\n        .unwrap_or_else(|err| panic!("LangErr {:?}", err));\n\n    return caller_balance - amount;\n} else {\n    return caller_balance;\n}\n')),(0,a.kt)("h3",{id:"implementation"},"Implementation"),(0,a.kt)("p",null,"The detector's implementation can be found at ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout/tree/main/detectors/reentrancy"},"this link"),"."))}p.isMDXComponent=!0}}]);