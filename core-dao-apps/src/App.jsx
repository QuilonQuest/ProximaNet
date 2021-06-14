import React from 'react';
import {Route, Switch} from 'react-router-dom';
import Dao from './Dao';
import {BrowserRouterHook} from './utils/use-router';
import NotFound from "./NotFound";

function App() {
  return (
    <BrowserRouterHook>
      <Switch>
        <Route exact path="/:dao" component={Dao}/>
        <Route path="/" component={Dao}/>
        <Route path="*">
          <NotFound/>
        </Route>
      </Switch>
    </BrowserRouterHook>
  )
}

export default App
