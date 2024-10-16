import{_ as r}from"./Nr-EHyLhZ3V.js";import{_ as p}from"./slidev/CodeBlockWrapper.vue_vue_type_script_setup_true_lang-21b4e1aE.js";import{o as d,c as g,k as h,e as i,l as k,m as y,aa as s,q as C,s as o,B as l}from"./modules/vue-Crn47hHj.js";import{I as c}from"./slidev/center-Bm8JpB-J.js";import{u as m,f as E}from"./slidev/context-MFTQElAV.js";import"./modules/unplugin-icons-VlmeoXPa.js";import"./index-DIPpyZM8.js";import"./modules/shiki-BLWRfs9A.js";const F={__name:"2_concurrency_parallelism.md__slidev_24",setup(A){const{$slidev:u,$nav:_,$clicksContext:t,$clicks:f,$page:B,$renderContext:v,$frontmatter:e}=m();return t.setup(),($,a)=>{const n=p,D=r;return d(),g(c,C(o(l(E)(l(e),23))),{default:h(()=>[a[1]||(a[1]=i("h1",null,"Message Passing",-1)),k(n,y({},{ranges:["all","3","5-10","12-13","15-17","all"]}),{default:h(()=>a[0]||(a[0]=[i("pre",{class:"shiki shiki-themes dark-plus light-plus slidev-code",style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000","--shiki-dark-bg":"#1E1E1E","--shiki-light-bg":"#FFFFFF"}},[i("code",{class:"language-rust"},[i("span",{class:"line"},[i("span",{style:{"--shiki-dark":"#569CD6","--shiki-light":"#0000FF"}},"fn"),i("span",{style:{"--shiki-dark":"#DCDCAA","--shiki-light":"#795E26"}}," main"),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},"() {")]),s(`
`),i("span",{class:"line"},[i("span",{style:{"--shiki-dark":"#6A9955","--shiki-light":"#008000"}},"    // mpsc: Multiple Producers, Single Consumer")]),s(`
`),i("span",{class:"line"},[i("span",{style:{"--shiki-dark":"#569CD6","--shiki-light":"#0000FF"}},"    let"),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}}," ("),i("span",{style:{"--shiki-dark":"#9CDCFE","--shiki-light":"#001080"}},"sender"),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},", "),i("span",{style:{"--shiki-dark":"#9CDCFE","--shiki-light":"#001080"}},"receiver"),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},") = "),i("span",{style:{"--shiki-dark":"#4EC9B0","--shiki-light":"#267F99"}},"mpsc"),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},"::"),i("span",{style:{"--shiki-dark":"#DCDCAA","--shiki-light":"#795E26"}},"channel"),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},"();")]),s(`
`),i("span",{class:"line"}),s(`
`),i("span",{class:"line"},[i("span",{style:{"--shiki-dark":"#4EC9B0","--shiki-light":"#267F99"}},"    thread"),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},"::"),i("span",{style:{"--shiki-dark":"#DCDCAA","--shiki-light":"#795E26"}},"spawn"),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},"("),i("span",{style:{"--shiki-dark":"#569CD6","--shiki-light":"#0000FF"}},"move"),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}}," || {")]),s(`
`),i("span",{class:"line"},[i("span",{style:{"--shiki-dark":"#C586C0","--shiki-light":"#AF00DB"}},"        for"),i("span",{style:{"--shiki-dark":"#9CDCFE","--shiki-light":"#001080"}}," message"),i("span",{style:{"--shiki-dark":"#569CD6","--shiki-light":"#0000FF"}}," in"),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}}," ["),i("span",{style:{"--shiki-dark":"#CE9178","--shiki-light":"#A31515"}},'"hi"'),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},", "),i("span",{style:{"--shiki-dark":"#CE9178","--shiki-light":"#A31515"}},'"from"'),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},", "),i("span",{style:{"--shiki-dark":"#CE9178","--shiki-light":"#A31515"}},'"the"'),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},", "),i("span",{style:{"--shiki-dark":"#CE9178","--shiki-light":"#A31515"}},'"thread"'),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},"] {")]),s(`
`),i("span",{class:"line"},[i("span",{style:{"--shiki-dark":"#9CDCFE","--shiki-light":"#001080"}},"            sender"),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},"."),i("span",{style:{"--shiki-dark":"#DCDCAA","--shiki-light":"#795E26"}},"send"),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},"("),i("span",{style:{"--shiki-dark":"#9CDCFE","--shiki-light":"#001080"}},"message"),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},")."),i("span",{style:{"--shiki-dark":"#DCDCAA","--shiki-light":"#795E26"}},"unwrap"),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},"();")]),s(`
`),i("span",{class:"line"},[i("span",{style:{"--shiki-dark":"#4EC9B0","--shiki-light":"#267F99"}},"            thread"),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},"::"),i("span",{style:{"--shiki-dark":"#DCDCAA","--shiki-light":"#795E26"}},"sleep"),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},"("),i("span",{style:{"--shiki-dark":"#4EC9B0","--shiki-light":"#267F99"}},"Duration"),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},"::"),i("span",{style:{"--shiki-dark":"#DCDCAA","--shiki-light":"#795E26"}},"from_secs"),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},"("),i("span",{style:{"--shiki-dark":"#B5CEA8","--shiki-light":"#098658"}},"1"),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},"));")]),s(`
`),i("span",{class:"line"},[i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},"        }")]),s(`
`),i("span",{class:"line"},[i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},"    });")]),s(`
`),i("span",{class:"line"}),s(`
`),i("span",{class:"line"},[i("span",{style:{"--shiki-dark":"#569CD6","--shiki-light":"#0000FF"}},"    let"),i("span",{style:{"--shiki-dark":"#9CDCFE","--shiki-light":"#001080"}}," message"),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}}," = "),i("span",{style:{"--shiki-dark":"#9CDCFE","--shiki-light":"#001080"}},"receiver"),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},"."),i("span",{style:{"--shiki-dark":"#DCDCAA","--shiki-light":"#795E26"}},"recv"),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},"()."),i("span",{style:{"--shiki-dark":"#DCDCAA","--shiki-light":"#795E26"}},"unwrap"),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},"();")]),s(`
`),i("span",{class:"line"},[i("span",{style:{"--shiki-dark":"#DCDCAA","--shiki-light":"#795E26"}},"    println!"),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},"("),i("span",{style:{"--shiki-dark":"#CE9178","--shiki-light":"#A31515"}},'"Got: {message}"'),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},");")]),s(`
`),i("span",{class:"line"}),s(`
`),i("span",{class:"line"},[i("span",{style:{"--shiki-dark":"#C586C0","--shiki-light":"#AF00DB"}},"    for"),i("span",{style:{"--shiki-dark":"#9CDCFE","--shiki-light":"#001080"}}," message"),i("span",{style:{"--shiki-dark":"#569CD6","--shiki-light":"#0000FF"}}," in"),i("span",{style:{"--shiki-dark":"#9CDCFE","--shiki-light":"#001080"}}," receiver"),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}}," {")]),s(`
`),i("span",{class:"line"},[i("span",{style:{"--shiki-dark":"#DCDCAA","--shiki-light":"#795E26"}},"        println!"),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},"("),i("span",{style:{"--shiki-dark":"#CE9178","--shiki-light":"#A31515"}},'"Got: {message}"'),i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},");")]),s(`
`),i("span",{class:"line"},[i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},"    }")]),s(`
`),i("span",{class:"line"},[i("span",{style:{"--shiki-dark":"#D4D4D4","--shiki-light":"#000000"}},"}")])])],-1)])),_:1},16),k(D)]),_:1},16)}}},M=F;export{M as default};
