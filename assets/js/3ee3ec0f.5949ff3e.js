"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[7584],{9613:(e,t,n)=>{n.d(t,{Zo:()=>u,kt:()=>m});var r=n(9496);function i(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function a(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){i(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function s(e,t){if(null==e)return{};var n,r,i=function(e,t){if(null==e)return{};var n,r,i={},o=Object.keys(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||(i[n]=e[n]);return i}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(i[n]=e[n])}return i}var l=r.createContext({}),c=function(e){var t=r.useContext(l),n=t;return e&&(n="function"==typeof e?e(t):a(a({},t),e)),n},u=function(e){var t=c(e.components);return r.createElement(l.Provider,{value:t},e.children)},d="mdxType",p={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},f=r.forwardRef((function(e,t){var n=e.components,i=e.mdxType,o=e.originalType,l=e.parentName,u=s(e,["components","mdxType","originalType","parentName"]),d=c(n),f=i,m=d["".concat(l,".").concat(f)]||d[f]||p[f]||o;return n?r.createElement(m,a(a({ref:t},u),{},{components:n})):r.createElement(m,a({ref:t},u))}));function m(e,t){var n=arguments,i=t&&t.mdxType;if("string"==typeof e||i){var o=n.length,a=new Array(o);a[0]=f;var s={};for(var l in t)hasOwnProperty.call(t,l)&&(s[l]=t[l]);s.originalType=e,s[d]="string"==typeof e?e:i,a[1]=s;for(var c=2;c<o;c++)a[c]=n[c];return r.createElement.apply(null,a)}return r.createElement.apply(null,n)}f.displayName="MDXCreateElement"},2471:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>l,contentTitle:()=>a,default:()=>p,frontMatter:()=>o,metadata:()=>s,toc:()=>c});var r=n(2564),i=(n(9496),n(9613));const o={},a="Iterators over indexing",s={unversionedId:"detectors/iterators-over-indexing",id:"detectors/iterators-over-indexing",title:"Iterators over indexing",description:"What it does",source:"@site/docs/detectors/19-iterators-over-indexing.md",sourceDirName:"detectors",slug:"/detectors/iterators-over-indexing",permalink:"/scout/docs/detectors/iterators-over-indexing",draft:!1,editUrl:"https://github.com/CoinFabrik/scout/docs/detectors/19-iterators-over-indexing.md",tags:[],version:"current",sidebarPosition:19,frontMatter:{},sidebar:"docsSidebar",previous:{title:"Avoid fromat! macro usage",permalink:"/scout/docs/detectors/avoid-format-string"},next:{title:"Contribute",permalink:"/scout/docs/contribute"}},l={},c=[{value:"What it does",id:"what-it-does",level:3},{value:"Why is this bad?",id:"why-is-this-bad",level:3},{value:"Example",id:"example",level:3},{value:"Implementation",id:"implementation",level:3}],u={toc:c},d="wrapper";function p(e){let{components:t,...n}=e;return(0,i.kt)(d,(0,r.Z)({},u,n,{components:t,mdxType:"MDXLayout"}),(0,i.kt)("h1",{id:"iterators-over-indexing"},"Iterators over indexing"),(0,i.kt)("h3",{id:"what-it-does"},"What it does"),(0,i.kt)("p",null,"It warns if for loop uses indexing instead of iterator. If the indexing goes to ",(0,i.kt)("inlineCode",{parentName:"p"},".len()")," it will not warn."),(0,i.kt)("h3",{id:"why-is-this-bad"},"Why is this bad?"),(0,i.kt)("p",null,"Accessing a vector by index is slower than using an iterator. Also, if the index is out of bounds, it will panic."),(0,i.kt)("h3",{id:"example"},"Example"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},"    #[ink(message)]\n    pub fn bad_indexing(&self){\n        for i in 0..3 {\n            foo(self.value[i]);\n        }\n    }\n")),(0,i.kt)("p",null,"Use instead:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},"   #[ink(message)]\n   pub fn iterator(&self) {\n       for item in self.value.iter() {\n            foo(self.value[i]);\n       }\n   }\n\n// or if its not iterable (with `in`, `iter` or `to_iter()`)\n\n   #[ink(message)]\n   pub fn index_to_len(&self){\n       for i in 0..self.value.len() {\n            foo(self.value[i]);\n       }\n")),(0,i.kt)("h3",{id:"implementation"},"Implementation"),(0,i.kt)("p",null,"The detector's implementation can be found at ",(0,i.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout/tree/main/detectors/iterators-over-indexing"},"this link"),"."))}p.isMDXComponent=!0}}]);