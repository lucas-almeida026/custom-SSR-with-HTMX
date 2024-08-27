const num = reactive(0);
const increase = ()=>{
    num.set((x)=>x + 1);
};
const decrease = ()=>{
    if (num.get() > 0) {
        num.set((x)=>x - 1);
    }
};
