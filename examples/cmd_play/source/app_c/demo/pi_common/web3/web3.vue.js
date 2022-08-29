_$pi.define("app_c/demo/pi_common/web3/web3.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./web3.vue.tpl", "pi_common/ui/main_root", "pi_common/ethers/web3", "pi_common/ethers/marketplace_utils", "pi_common/ethers/index"], function (require, exports, module, direct_1, web3_vue_tpl_1, main_root_1, web3, marketplaceUtils, index_1) {
    "use strict";

    exports.initMeta = void 0;
    class Web3Demo {
        constructor() {
            this.list = [['初始化', () => {
                web3.init().then(() => {
                    console.log('init success');
                });
            }], ['获取链ID', () => {
                web3.getChainId().then(r => {
                    console.log('getChainId = ', r);
                });
            }], ['获取链接账号', () => {
                web3.getAccounts().then(r => {
                    console.log('getAccounts = ', r);
                });
            }], ['监听事件', () => {
                web3.addEventListener('accountsChanged', args => {
                    console.log('accountsChanged ', args);
                });
                web3.addEventListener('chainChanged', args => {
                    console.log('chainChanged ', args);
                });
                web3.addEventListener('connect', args => {
                    console.log('connect ', args);
                });
                web3.addEventListener('disconnect', args => {
                    console.log('disconnect ', args);
                });
            }], ['切换指定chainId', () => {
                web3.switchEthereumChain('0x1').then(r => {
                    console.log('switchEthereumChain = ', r);
                });
            }], ['添加新链', () => {
                web3.addEthereumChain({
                    chainId: '0x61',
                    chainName: 'Smart Chain - Testnet',
                    rpcUrls: ['https://data-seed-prebsc-1-s1.binance.org:8545/E'],
                    nativeCurrency: {
                        name: 'Testnet Binance',
                        symbol: 'BNB',
                        decimals: 18
                    },
                    blockExplorerUrls: ['https://testnet.bscscan.com']
                }).then(r => {
                    console.log('addEthereumChain = ', r);
                });
            }], ['添加代币', () => {
                web3.watchAsset({
                    type: 'ERC20',
                    options: {
                        address: '0xeB4dAf1d482d445685bC8dFb8F404A16a267AE6F',
                        symbol: 'GROK',
                        decimals: 18,
                        image: 'https://wwwtest.grok.earth/assets/svg_new/grok_icon.svg'
                    }
                }).then(r => {
                    console.log('watchAsset = ', r);
                });
            }], ['转账', () => {
                const tx = {
                    to: '0x0a089EF7c57808F91539b9f5ba351bC37414D023',
                    value: index_1.ethers.utils.parseUnits('0.001')
                };
                web3.transfer(tx).then(r => {
                    console.log('transfer = ', r);
                });
            }], ['签名消息', () => {
                web3.signMessage('abcdefg').then(r => {
                    console.log('signMessage = ', r);
                });
            }], ['获取主币余额', () => {
                web3.getMainBalance('0x32047eDB3d9572e5D14613b7b7F956890798323f').then(r => {
                    console.log('getMainBalance = ', r);
                });
            }], ['监听交易hash', () => {
                web3.waitForTransaction('0x7af906bc938711c6c5f73e8f60b6f48b0b6cc43534bf893e2957cf6ee3692bf1').then(r => {
                    console.log('waitForTransaction = ', r);
                });
            }], ['获取ERC20代币余额', () => {
                web3.ERC20.getBalance('0x32047eDB3d9572e5D14613b7b7F956890798323f', '0xeB4dAf1d482d445685bC8dFb8F404A16a267AE6F').then(r => {
                    console.log('ERC20.getBalance = ', r);
                });
            }], ['ERC20代币是否需要授权', () => {
                web3.ERC20.needApproval('0x32047eDB3d9572e5D14613b7b7F956890798323f', '0xeB4dAf1d482d445685bC8dFb8F404A16a267AE6F', '0x9C47B682b497803BB4A000eE079030077A71848F').then(r => {
                    console.log('ERC20.needApproval = ', r);
                });
            }], ['ERC20代币授权', () => {
                web3.ERC20.approval('0xeB4dAf1d482d445685bC8dFb8F404A16a267AE6F', '0x9C47B682b497803BB4A000eE079030077A71848F').then(r => {
                    console.log('ERC20.approval = ', r);
                });
            }], ['货币格式化', () => {
                console.log('formatCurrency = ', marketplaceUtils.formatCurrency(99999999.4567874134));
            }], ['地址格式化', () => {
                console.log('formatAddress = ', marketplaceUtils.formatAddress('0xeB4dAf1d482d445685bC8dFb8F404A16a267AE6F'));
            }], ['时间格式化', () => {
                console.log('formatTime = ', marketplaceUtils.formatTime(Date.now()));
            }], ['获取当前UTC时间', () => {
                console.log('getUTCnow = ', marketplaceUtils.getUTCnow());
            }], ['北京时间转UTC时间', () => {
                console.log('beijing2UTCtime = ', marketplaceUtils.beijing2UTCtime(Date.now()));
            }]];
            this.closePage = () => {
                main_root_1.close(this);
            };
        }
        click(i) {
            const func = this.list[i][1];
            if (func) {
                func();
            }
        }
    }
    exports.default = Web3Demo;
    exports.initMeta = () => {
        let _$tpl = "app_c/demo/pi_common/web3/web3.vue.tpl.ts",
            _$cssPath = "app_c/demo/pi_common/web3/web3.vue.wcss",
            _$cssHash = 1848019445;
        Web3Demo["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: web3_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(Web3Demo, ["list"]);
    direct_1.addField(Web3Demo, ['closePage']);
});