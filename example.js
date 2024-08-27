const num = reactive(0);
const increase = ()=>{
    num.set((x)=>x + 1);
};
const decrease = ()=>{
    if (num.get() > 0) {
        num.set((x)=>x - 1);
    }
};
export default function MyComponent() {
    return (<div>
			<h1>This is my counter!</h1>
			<ControllBtn data-scid="e3abfdc1" text="-" onClick={decrease}/>
			<p>{num.get()}</p>
			<ControllBtn data-scid="ddfe13fc" text="+" onClick={increase}/>
		</div>);
}
function ControllBtn({ onClick, text }) {
    return (<button className="btn btn-primary btn-filled" onClick={onClick}>{text}</button>);
}

/*my_component_d920c16a.js
const d920c16a_num = reactive(0);
const d920c16a_increase = ()=>{
    num.set((x)=>x + 1);
};
const d920c16a_decrease = ()=>{
    if (num.get() > 0) {
        num.set((x)=>x - 1);
    }
};
const e3abfdc1_control_btn = document.querySelector('[data-scid="e3abfdc1"]');
const ddfe13fc_control_btn = document.querySelector('[data-scid="ddfe13fc"]');
e3abfdc1_control_btn.addEventListener('click', d920c16a_decrease);
ddfe13fc_control_btn.addEventListener('click', d920c16a_increase);
*/

/*my_component.html_template
<div>
	<h1>This is my counter!</h1>
	{children[0]}
	<p>{num.get()}</p>
	{children[1]}
</div>
*/

/*control_btn.html_template
<button className="btn btn-primary btn-filled">{text}</button>
*/