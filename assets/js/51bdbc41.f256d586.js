"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[6516],{9613:(e,t,n)=>{n.d(t,{Zo:()=>u,kt:()=>h});var a=n(9496);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function l(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?l(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):l(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function c(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},l=Object.keys(e);for(a=0;a<l.length;a++)n=l[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var l=Object.getOwnPropertySymbols(e);for(a=0;a<l.length;a++)n=l[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var o=a.createContext({}),s=function(e){var t=a.useContext(o),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},u=function(e){var t=s(e.components);return a.createElement(o.Provider,{value:t},e.children)},p="mdxType",m={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},d=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,l=e.originalType,o=e.parentName,u=c(e,["components","mdxType","originalType","parentName"]),p=s(n),d=r,h=p["".concat(o,".").concat(d)]||p[d]||m[d]||l;return n?a.createElement(h,i(i({ref:t},u),{},{components:n})):a.createElement(h,i({ref:t},u))}));function h(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var l=n.length,i=new Array(l);i[0]=d;var c={};for(var o in t)hasOwnProperty.call(t,o)&&(c[o]=t[o]);c.originalType=e,c[p]="string"==typeof e?e:r,i[1]=c;for(var s=2;s<l;s++)i[s]=n[s];return a.createElement.apply(null,i)}return a.createElement.apply(null,n)}d.displayName="MDXCreateElement"},538:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>o,contentTitle:()=>i,default:()=>m,frontMatter:()=>l,metadata:()=>c,toc:()=>s});var a=n(2564),r=(n(9496),n(9613));const l={},i="Reentrancy",c={unversionedId:"vulnerabilities/reentrancy",id:"vulnerabilities/reentrancy",title:"Reentrancy",description:"Description",source:"@site/docs/vulnerabilities/3-reentrancy.md",sourceDirName:"vulnerabilities",slug:"/vulnerabilities/reentrancy",permalink:"/scout/docs/vulnerabilities/reentrancy",draft:!1,editUrl:"https://github.com/CoinFabrik/scout/docs/vulnerabilities/3-reentrancy.md",tags:[],version:"current",sidebarPosition:3,frontMatter:{},sidebar:"docsSidebar",previous:{title:"Set contract storage",permalink:"/scout/docs/vulnerabilities/set-contract-storage"},next:{title:"Panic Error",permalink:"/scout/docs/vulnerabilities/panic-error"}},o={},s=[{value:"Description",id:"description",level:2},{value:"Exploit Scenario",id:"exploit-scenario",level:2},{value:"Deployment",id:"deployment",level:3},{value:"Recommendation",id:"recommendation",level:2},{value:"References",id:"references",level:2}],u={toc:s},p="wrapper";function m(e){let{components:t,...n}=e;return(0,r.kt)(p,(0,a.Z)({},u,n,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("h1",{id:"reentrancy"},"Reentrancy"),(0,r.kt)("h2",{id:"description"},"Description"),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},"Vulnerability Category: ",(0,r.kt)("inlineCode",{parentName:"li"},"Reentrancy")),(0,r.kt)("li",{parentName:"ul"},"Severity: ",(0,r.kt)("inlineCode",{parentName:"li"},"Critical")),(0,r.kt)("li",{parentName:"ul"},"Detectors: ",(0,r.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout/tree/main/detectors/reentrancy-1"},(0,r.kt)("inlineCode",{parentName:"a"},"reentrancy-1")),", ",(0,r.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout/tree/main/detectors/reentrancy-2"},(0,r.kt)("inlineCode",{parentName:"a"},"reentrancy-2"))),(0,r.kt)("li",{parentName:"ul"},"Test Cases: ",(0,r.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout/tree/main/test-cases/reentrancy-1/reentrancy-1"},(0,r.kt)("inlineCode",{parentName:"a"},"reentrancy-1")),", ",(0,r.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout/tree/main/test-cases/reentrancy-1/reentrancy-2"},(0,r.kt)("inlineCode",{parentName:"a"},"reentrancy-2")),", ",(0,r.kt)("a",{parentName:"li",href:"https://github.com/CoinFabrik/scout/tree/main/test-cases/reentrancy-2/reentrancy-1"},(0,r.kt)("inlineCode",{parentName:"a"},"reentrancy-3")))),(0,r.kt)("p",null,"Smart contracts can call other contracts and send tokens to them. These\noperations imply external calls where control flow is passed to the called\ncontract until the execution of the called code is over. Then the control\nis delivered back to the caller."),(0,r.kt)("p",null,"External calls, therefore, could open the opportunity for a malicious contract\nto execute any arbitrary code. This includes calling back the caller contract,\nan attack known as reentrancy. This kind of attack was used in Ethereum for\nthe infamous ",(0,r.kt)("a",{parentName:"p",href:"https://blog.chain.link/reentrancy-attacks-and-the-dao-hack/"},"DAO Hack"),"."),(0,r.kt)("h2",{id:"exploit-scenario"},"Exploit Scenario"),(0,r.kt)("p",null,"In order to exemplify this vulnerability we developed two contracts:\na ",(0,r.kt)("inlineCode",{parentName:"p"},"Vault")," contract and an ",(0,r.kt)("inlineCode",{parentName:"p"},"Exploit")," contract."),(0,r.kt)("p",null,"The ",(0,r.kt)("inlineCode",{parentName:"p"},"Vault")," contract provides functions to deposit, withdraw, check balance,\nand call a function on another contract with a specified value."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'#[ink(message)]\npub fn call_with_value(&mut self, address: AccountId, amount: Balance, selector: u32) -> Balance {\n    ink::env::debug_println!("call_with_value function called from {:?}",self.env().caller());\n    let caller_addr = self.env().caller();\n    let caller_balance = self.balances.get(caller_addr).unwrap_or(0);\n    if amount <= caller_balance {\n        let call = build_call::<ink::env::DefaultEnvironment>()\n            .call(address)\n            .transferred_value(amount)\n            .exec_input(\n                ink::env::call::ExecutionInput::new(Selector::new(selector.to_be_bytes()))\n            )\n            .call_flags(\n                ink::env::CallFlags::default()\n                    .set_allow_reentry(true)\n            )\n            .returns::<()>()\n            .params();\n        self.env().invoke_contract(&call)\n            .unwrap_or_else(|err| panic!("Err {:?}",err))\n            .unwrap_or_else(|err| panic!("LangErr {:?}",err));\n        self.balances.insert(caller_addr, &(caller_balance - amount));\n\n        return caller_balance - amount;\n    } else {\n        return caller_balance\n    }\n}\n')),(0,r.kt)("p",null,"Th function ",(0,r.kt)("inlineCode",{parentName:"p"},"call_with_value function()")," allows the contract owner to call\nother contracts on the blockchain and transfer a specified amount of value in\nthe process. The function takes three arguments:"),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("em",{parentName:"li"},"address"),": The address of the contract to call."),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("em",{parentName:"li"},"amount"),": The amount of balance to transfer in the call."),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("em",{parentName:"li"},"selector"),": The 32-bit function selector of the function to call on the contract.")),(0,r.kt)("p",null,"The function first checks the balance of the caller to make sure that they have\nenough funds to perform the transfer. If the balance is sufficient, a new call\nis constructed using the ",(0,r.kt)("inlineCode",{parentName:"p"},"build_call()")," function provided by the\n",(0,r.kt)("inlineCode",{parentName:"p"},"env::call module"),"."),(0,r.kt)("p",null,"The ",(0,r.kt)("inlineCode",{parentName:"p"},"build_call()")," function constructs a new contract call with the specified\narguments. In this case, the call method is used to specify the address of the\ncontract to call, the transferred_value method is used to specify the amount\nof balance to transfer, and the exec_input method is used to specify the\nfunction selector and any arguments to pass to the called function."),(0,r.kt)("p",null,"The ",(0,r.kt)("inlineCode",{parentName:"p"},"call_flags()")," method is also used to set a flag that allows the called\ncontract to re-enter the current contract if necessary. This possibility to\nre-enter the contract, together with an appropriate 32-bit function selector\nwill allow us to repeatedly withdraw balance from the contract, emptying the\nVault."),(0,r.kt)("p",null,"In order to perform this attack, we will use the ",(0,r.kt)("inlineCode",{parentName:"p"},"exploit()")," function of the\n",(0,r.kt)("inlineCode",{parentName:"p"},"Exploit")," contract that we outline below:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'#[ink(message, payable, selector = 0x0)]\npub fn exploit(&mut self) {\n    ink::env::debug_println!("Exploit  function called from {:?} gas left {:?}",self.env().caller(), self.env().gas_left());\n    if self.env().gas_left() > self.gas_to_stop{\n        let call = build_call::<ink::env::DefaultEnvironment>()\n        .call(self.contract)\n        .transferred_value(0)\n        .exec_input(\n            ink::env::call::ExecutionInput::new(Selector::new([0x76_u8,0x75_u8,0x7E_u8,0xD3_u8]))\n                .push_arg(self.env().account_id())\n                .push_arg(self.amount)\n                .push_arg(0)\n        )\n        .call_flags(\n            ink::env::CallFlags::default()\n                .set_allow_reentry(true)\n        )\n        .returns::<Balance>()\n        .params();\n        ink::env::debug_println!("Call generated gas left:{:?}",self.env().gas_left());\n        self.env().invoke_contract(&call)\n            .unwrap_or_else(|err| panic!("Err {:?}",err))\n            .unwrap_or_else(|err| panic!("LangErr {:?}",err));\n        ink::env::debug_println!("Call finished");\n    }\n}\n\n')),(0,r.kt)("p",null,"The vulnerable code example can be found ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout/tree/main/test-cases/reentrancy-1/reentrancy-1/vulnerable-example"},"here"),"."),(0,r.kt)("h3",{id:"deployment"},"Deployment"),(0,r.kt)("p",null,"Vault and Exploit files can be found under the directories\n",(0,r.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout/tree/main/test-cases/reentrancy-1/reentrancy-1/vulnerable-example/exploit"},"vulnerable-example/exploit")," and\n",(0,r.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout/tree/main/test-cases/reentrancy-1/reentrancy-1/vulnerable-example/vault"},"vulnerable-example/vault"),".\nThe whole exploit example can be run automatically using the ",(0,r.kt)("inlineCode",{parentName:"p"},"deploy.sh")," file."),(0,r.kt)("h2",{id:"recommendation"},"Recommendation"),(0,r.kt)("p",null,"In general, risks associated to reentrancy can be addressed with the\nCheck-Effect-Interaction pattern, a best practice that indicates that external\ncalls should be the last thing to be executed in a function. In this example,\nthis can be done by inserting the balance before transferring the value (see\n",(0,r.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout/tree/main/test-cases/reentrancy-1/reentrancy-1/remediated-example"},"remediated-example-1"),")."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'pub fn call_with_value(&mut self, address: AccountId, amount: Balance, selector: u32) -> Balance {\n    ink::env::debug_println!("call_with_value function called from {:?}",self.env().caller());\n    let caller_addr = self.env().caller();\n    let caller_balance = self.balances.get(caller_addr).unwrap_or(0);\n    if amount <= caller_balance {\n        self.balances.insert(caller_addr, &(caller_balance - amount));\n        let call = build_call::<ink::env::DefaultEnvironment>()\n            .call(address)\n            .transferred_value(amount)\n            .exec_input(\n                ink::env::call::ExecutionInput::new(Selector::new(selector.to_be_bytes()))\n            )\n            .call_flags(\n                ink::env::CallFlags::default()\n                    .set_allow_reentry(true)\n            )\n            .returns::<()>()\n            .params();\n        self.env().invoke_contract(&call)\n            .unwrap_or_else(|err| panic!("Err {:?}",err))\n            .unwrap_or_else(|err| panic!("LangErr {:?}",err));\n\n        return caller_balance - amount;\n    } else {\n        return caller_balance\n    }\n}\n')),(0,r.kt)("p",null,"The remediated code example can be found ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout/tree/main/test-cases/reentrancy-1/reentrancy-1/remediated-example"},"here"),"."),(0,r.kt)("p",null,"Alternatively, if reentrancy by an external contract is not needed, the\n",(0,r.kt)("inlineCode",{parentName:"p"},"set_allow_reentry(true)")," should be removed altogether (see\n",(0,r.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout/tree/main/test-cases/reentrancy-1/reentrancy-2/remediated-example"},"remediated-example-2"),"). This is equivalent in Substrate to using a\n",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/security/reentrancy_guard"},"reentrancy guard"),"\nlike the one offered by ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts"},"OpenBrush"),"."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'#[ink(message)]\npub fn call_with_value(&mut self, address: AccountId, amount: Balance, selector: u32) -> Balance {\n    ink::env::debug_println!("call_with_value function called from {:?}",self.env().caller());\n    let caller_addr = self.env().caller();\n    let caller_balance = self.balances.get(caller_addr).unwrap_or(0);\n    if amount <= caller_balance {\n        let call = build_call::<ink::env::DefaultEnvironment>()\n            .call(address)\n            .transferred_value(amount)\n            .exec_input(\n                ink::env::call::ExecutionInput::new(Selector::new(selector.to_be_bytes()))\n            )\n            .returns::<()>()\n            .params();\n        self.env().invoke_contract(&call)\n            .unwrap_or_else(|err| panic!("Err {:?}",err))\n            .unwrap_or_else(|err| panic!("LangErr {:?}",err));\n        self.balances.insert(caller_addr, &(caller_balance - amount));\n\n        return caller_balance - amount;\n    } else {\n        return caller_balance\n    }\n}\n')),(0,r.kt)("p",null,"The remediated code example can be found ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout/tree/main/test-cases/reentrancy-1/reentrancy-2/remediated-example"},"here"),"."),(0,r.kt)("h2",{id:"references"},"References"),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("a",{parentName:"li",href:"https://use.ink/datastructures/storage-layout"},"https://use.ink/datastructures/storage-layout")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("a",{parentName:"li",href:"https://consensys.github.io/smart-contract-best-practices/attacks/reentrancy/"},"https://consensys.github.io/smart-contract-best-practices/attacks/reentrancy/")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("a",{parentName:"li",href:"https://dasp.co/#item-1"},"https://dasp.co/#item-1")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("a",{parentName:"li",href:"https://blog.sigmaprime.io/solidity-security.html#SP-1"},"https://blog.sigmaprime.io/solidity-security.html#SP-1")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("a",{parentName:"li",href:"https://docs.soliditylang.org/en/develop/security-considerations.html#re-entrancy"},"https://docs.soliditylang.org/en/develop/security-considerations.html#re-entrancy")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("a",{parentName:"li",href:"https://stermi.medium.com/the-ethernaut-challenge-9-solution-re-entrancy-635303881a4f"},"Ethernaut: Reentrancy")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("a",{parentName:"li",href:"https://swcregistry.io/docs/SWC-107"},"SWC-107")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("a",{parentName:"li",href:"https://github.com/crytic/slither/wiki/Detector-Documentation#reentrancy-vulnerabilities"},"Slither: Reentrancy vulnerabilities (theft of ethers)")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("a",{parentName:"li",href:"https://github.com/crytic/slither/wiki/Detector-Documentation#reentrancy-vulnerabilities-1"},"Slither: Reentrancy vulnerabilities (no theft of ethers)")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("a",{parentName:"li",href:"https://github.com/crytic/slither/wiki/Detector-Documentation#reentrancy-vulnerabilities-2"},"Slither: Benign reentrancy vulnerabilities")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("a",{parentName:"li",href:"https://github.com/crytic/slither/wiki/Detector-Documentation#reentrancy-vulnerabilities-3"},"Slither: Reentrancy vulnerabilities leading to out-of-order Events")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("a",{parentName:"li",href:"https://github.com/crytic/slither/wiki/Detector-Documentation#reentrancy-vulnerabilities-4"},"Slither: Reentrancy vulnerabilities through send and transfer"))))}m.isMDXComponent=!0}}]);