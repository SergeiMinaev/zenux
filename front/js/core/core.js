import { r, run } from '../../ext/fr9/fr9.js';
import { Home } from './home.js';


class App {
	rootNode = 'app';
	tpl = 't-app';
	components = {
		'home': Home,
	}
}
window.app = new App();
run(window.app);
