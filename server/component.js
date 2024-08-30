const MyButton = require('./myButton.js')
const html = '<div><h1>{% pageTitle }</h1>{#[0]}</div>'
const Page = {
	load: (data) => {
		const children = html.matchAll(/\{\#\[\d+\]\}/g)
		console.log(children.next().value)
		return html.replaceAll('{% text }', data?.text ?? '');
	}
}
module.exports = Page