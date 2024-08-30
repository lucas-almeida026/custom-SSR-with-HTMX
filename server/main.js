// const Page = require('./component.js')

// const rendered = Page.load({text: 'asdf', btnText: 'gg'})
// console.log(rendered)

/* server.js
import express from 'express';
import db from './db';
import {IndexPage} from './pages';

const app = express();


app.get('/', async (req, res) => {
	const bannerURLs = await db.getBannerURLs();
	res.send(<IndexPage bannerURLs={bannerURLs} />);
});
app.listen(3000);
*/

/* IndexPage.jsx
const selected = reactive(0);
export default function IndexPage({ bannerURLs }) {
	return (
		<main className="d-flex flex-col">
			@iter bannerURLs: url, i {
				<Banner url={url} blured={selected.get() == i} />
			}

		</main>
	)
}
*/

/* Banner.jsx
export default function Banner({ url, blured }) {
	return (
		<div className="d-flex flex-col p-3 rounded">
			<img src={url} className={blured ? 'blur-sm' : ''} onClick={toggleBlur/>
		</div>
	)
}
*/

//-------
