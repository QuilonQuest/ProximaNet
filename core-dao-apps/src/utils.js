import {connect, Contract, keyStores, WalletConnection} from 'near-api-js';
import {useGlobalState, useGlobalMutation} from './utils/container'
import 'regenerator-runtime/runtime';

import getConfig from './config'
const nearConfig = getConfig(process.env.NODE_ENV || 'development')

export async function initContract() {
  const near = await connect(Object.assign({deps: {keyStore: new keyStores.BrowserLocalStorageKeyStore()}}, nearConfig))
  window.walletConnection = new WalletConnection(near)
  window.accountId = window.walletConnection.getAccountId()
  window.factoryContract = await new Contract(window.walletConnection.account(), nearConfig.contractName, {
    viewMethods: ['get_dao_list'],
    changeMethods: [],
  })
}

export function logout() {
  window.walletConnection.signOut()
  window.location.replace(window.location.origin + window.location.pathname)
}

export function login() {
  window.walletConnection.requestSignIn(nearConfig.contractName)
}
