import{d as a,z as l,o as c,b as i,e as d,f as u,h as f}from"../modules/vue-C7l4V3iz.js";import{u as p}from"./context-B50feDHT.js";function n(e){return e.startsWith("/")?"/rust-workshop/1/"+e.slice(1):e}function m(e,s=!1){const o=e&&["#","rgb","hsl"].some(r=>e.indexOf(r)===0),t={background:o?e:void 0,color:e&&!o?"white":void 0,backgroundImage:o?void 0:e?s?`linear-gradient(#0005, #0008), url(${n(e)})`:`url("${n(e)}")`:void 0,backgroundRepeat:"no-repeat",backgroundPosition:"center",backgroundSize:"cover"};return t.background||delete t.background,t}const v={class:"my-auto w-full"},g=a({__name:"cover",props:{background:{default:""}},setup(e){p();const s=e,o=l(()=>m(s.background,!0));return(t,r)=>(c(),i("div",{class:"slidev-layout cover",style:f(o.value)},[d("div",v,[u(t.$slots,"default")])],4))}});export{g as _};
