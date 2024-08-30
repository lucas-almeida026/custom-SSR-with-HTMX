const html = '<button>{% text }</button>'
const MyButton = {
	load: (data) => html.replaceAll('{% text }', data?.text ?? '')
}
module.exports = MyButton