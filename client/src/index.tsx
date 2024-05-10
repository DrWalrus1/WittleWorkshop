import { render } from 'solid-js/web';
import { Router, Route } from "@solidjs/router";

import './index.css';
import Home from './Home';
import Docker from './Docker';
import Services from './Services';
import Navbar from './Navbar';

const root = document.getElementById('root');

if (import.meta.env.DEV && !(root instanceof HTMLElement) && root !== null) {
  throw new Error(
    'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got misspelled?',
  );
}

const App = (props: any) => (
  <>
    <Navbar />
    <div class="core">
      {props.children}
    </div>
  </>
)

render(() => (
  <Router root={App}>
    <Route path="/" component={Home} />
    <Route path="/managedocker" component={Docker} />
    <Route path="/manageservices" component={Services} />
  </Router>
), document.getElementById('root')!);
