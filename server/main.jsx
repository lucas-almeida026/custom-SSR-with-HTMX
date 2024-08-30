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
			<img src={url} className={blured ? 'blur-sm' : ''} onClick={toggleBlur}/>
		</div>
	)
}
*/

//-------
const blur = reactive(false);
const toggleBlur = () => blur.set(x => !x);
<div className="d-flex flex-col p-3 rounded">
	<img src={url} className={blured ? 'blur-sm' : ''} onClick={toggleBlur}/>
</div>

/* component.html
<div class="d-flex flex-col p-3 rounded">
	<img src={{url}} class="{{if blured}} blur-sm {{end}}" data-ssr-id="{{ssrid}}" />
</div>
*/
/* component.js
const img_hasdf8970yh = document.querySelector('img[data-ssr-id="hasdf8970yh"]');
img_hasdf8970yh.onclick = toggleBlur;
*/
/* bundle.js
const blur_nvsd9q7uh3w = reactive(false);
const toggleBlur = () => blur_nvsd9q7uh3w.set(x => !x);
*/