### Simple Component
```jsx
export default function MyTitle() {
	return <h1>My title</h1>
}
```
---
### Component with simple props
```jsx
export default function MyButton({ text, disabled, onClick }) {
	return <button disabled={disabled} onClick={onClick}>{text}</button>
}
```
---
### Component with child component
```jsx
export default function MyComponent() {
	return (
		<div>
			<MyTitle />
			<p>My text</p>
		</div>
	)
}
```
---
### Component state + child component
```jsx
const num = reactive(0);
const increase = () => {
	num.set(x => x + 1);
}
const decrease = () => {
	if (num.get() > 0) {
		num.set(x => x - 1);
	}
}
export default function MyComponent() {
	return (
		<div>
			<h1>This is my counter!</h1>
			<ControllBtn text="-" onClick={decrease} />
			<p>{num.get()}</p>
			<ControllBtn text="+" onClick={increase} />
		</div>
	)
}
```