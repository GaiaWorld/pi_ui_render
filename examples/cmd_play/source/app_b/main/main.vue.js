_$pi.define("app_b/main/main.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./main.vue.tpl", "app_a/login/server/user.topic", "app_a/struct/db_role.struct", "app_a/util/db_tool", "app_a/util/setup", "app_b/meteor/main", "pi_common/store/store", "pi_common/ui/main_root", "app_a/audio_test/audio_test.vue", "pi_utils/util/logger", "pi_common/ui/main_root", "app_b/main_container/main_container.vue", "app_a/native_test/native_test.vue", "pi_common/client_net/network", "pi_common/client_net/constant", "app_c/demo/constant", "app_c/demo/ui/menu/root.vue", "app_c/demo/demo_single.vue", "pi_common/ui/hot_key"], function (require, exports, module, direct_1, main_vue_tpl_1, user_topic_1, db_role_struct_1, db_tool_1, setup_1, main_1, store_1, main_root_1, audio_test_vue_1, logger_1, main_root_2, main_container_vue_1, native_test_vue_1, network_1, constant_1, constant_2, root_vue_1, demo_single_vue_1, hot_key_1) {
    "use strict";

    exports.initMeta = void 0;
    const { logV, logD, logI, logW, logE } = logger_1.getLogger(module.name, 'app');
    class Main {
        constructor() {
            this.barOffset = 0;
            this.gold = 0;
            this.offline = false;
            this.btnList = [['pi_common', () => {
                main_root_1.open(demo_single_vue_1.default, { list: constant_2.PiCommonCfgList });
            }], ['pi_gui', () => {
                main_root_1.open(root_vue_1.default, { showDir: 'app_c/demo/ui/show/', sub: '' });
            }], ['服务器', () => {
                main_root_1.open(demo_single_vue_1.default, { list: constant_2.ServerCfgList });
            }]];
            this.beforeSlideOut = () => {
                main_1.pauseMeteor();
            };
            this.afterSlideIn = () => {
                main_1.activeMeteor();
            };
            this.addMoney = () => {
                setup_1.clientRpc(user_topic_1.collectMoney, null);
            };
            this.audioClick = () => {
                main_root_1.open(audio_test_vue_1.default);
            };
            this.nativeClick = () => {
                main_root_1.open(native_test_vue_1.default);
            };
        }
        mouseClick(e) {
            console.log('======== mouseClick =======', e);
            if (!this.currentNode) {
                this.currentNode = e.current;
                this.currentNode.document.addEventListener('lockchange', e => {
                    console.log('======== lockchange =======', this.currentNode.document.pointerLockElement);
                });
                hot_key_1.registerHotKey('Enter', () => {
                    this.currentNode.requestPointerLock();
                });
                hot_key_1.registerHotKey('a', () => {
                    this.currentNode.document.exitPointerLock();
                });
            }
            // currentNode.requestPointerLock();
        }
        mouseMove(e) {
            console.log('======== mouseMove ', e);
        }
        mouseOver(e) {
            // console.log('======== mouseOver ', e);
        }
        mouseOut(e) {
            // console.log('======== mouseOut ', e);
        }
        mouseEnter(e) {
            // console.log('======== mouseEnter ', e);
        }
        mouseLeave(e) {
            // console.log('======== mouseLeave ', e);
        }
        childMouseOver(e) {
            // console.log('======== childMouseOver ', e);
            return true;
        }
        childMouseOut(e) {
            // console.log('======== childMouseOut ', e);
            return true;
        }
        childMouseEnter(e) {
            // console.log('======== childMouseEnter ', e);
            return true;
        }
        childMouseLeave(e) {
            // console.log('======== childMouseLeave ', e);
            return true;
        }
        create() {
            this._$info.document.eventMgr.startMoveCheckLoop();
            main_1.activeMeteor();
            this.barOffset = 0;
            let db = store_1.find(db_role_struct_1.CurrencyDb);
            this.gold = db ? db_tool_1.getGold(db) : 0;
            store_1.register(db_role_struct_1.CurrencyDb, r => {
                this.gold = db_tool_1.getGold(r);
            });
            main_container_vue_1.WIDGET_MAP.set(Main, { _sceneName: 'MAIN_SCENE', _widget: this });
            network_1.Network.netEventBus.add(constant_1.NET_STATUS.AUTH, () => {
                console.log(`%c 我已经连上网了=======`, `color:red;font-size:20px`);
                this.offline = false;
            });
            network_1.Network.netEventBus.add(constant_1.NET_STATUS.DISCONNECT, () => {
                console.log(`%c 我断开了=======`, `color:red;font-size:20px`);
                this.offline = true;
            });
        }
        // 当进入后直接切到其他界面会找不到位置
        attach() {
            // 配合底部菜单使用
            main_container_vue_1.SCREEN_HANDLER.notify(main_container_vue_1.ScreenEvent.WidgetPainted, [Main]);
            main_container_vue_1.toggleBottomMenu(true);
        }
        exitGame() {
            logD('exitGame OK');
            const root = main_root_2.getRoot();
            if (root.backList.length) {
                root.closeBack();
            }
            setup_1.openLoginTpl();
            setup_1.Global.sdk.login.logout();
        }
        click(num) {
            let item = this.btnList[num];
            item[1]();
        }
    }
    exports.default = Main;
    exports.initMeta = () => {
        let _$tpl = "app_b/main/main.vue.tpl.ts",
            _$cssPath = "app_b/main/main.vue.wcss",
            _$cssHash = 1942207890;
        Main["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: main_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(Main, ["barOffset", "gold", "btnList", "offline"]);
    direct_1.addField(Main, ['currentNode', 'beforeSlideOut', 'afterSlideIn', 'addMoney', 'audioClick', 'nativeClick']);
});