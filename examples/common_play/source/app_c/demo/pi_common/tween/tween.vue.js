_$pi.define("app_c/demo/pi_common/tween/tween.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./tween.vue.tpl", "pi_common/ui/main_root", "pi_gui/widget/direct", "pi_common/tween/index", "pi_common/tween/setup"], function (require, exports, module, direct_1, tween_vue_tpl_1, main_root_1, direct_2, index_1, setup_1) {
    "use strict";

    exports.initMeta = void 0;
    class TweenDemo {
        constructor() {
            this.closePage = () => {
                main_root_1.close(this);
            };
        }
        create() {
            // 初始化tween模块 只应该执行一次
            setup_1.startTweenLoop();
        }
        click() {
            console.log('点中了。。。');
            this.turnMoveClose(() => {
                this.turnMoveOpen();
            });
        }
        turnMoveClose(func) {
            let node1 = direct_2.findElementByAttr(this, "id", `test`);
            let node = new index_1.Node(node1);
            index_1.tween(node).set({ scale: { x: 1, y: 1 } }).delay(0).to(0.5, { scale: { x: 0, y: 1 } }).call(func).start();
        }
        turnMoveOpen() {
            let node1 = direct_2.findElementByAttr(this, "id", `test`);
            let node = new index_1.Node(node1);
            index_1.tween(node).set({ scale: { x: 0, y: 1 } }).delay(0).to(0.5, { scale: { x: 1, y: 1 } }).call(null).start();
        }
    }
    exports.default = TweenDemo;
    exports.initMeta = () => {
        let _$tpl = "app_c/demo/pi_common/tween/tween.vue.tpl.ts",
            _$cssPath = "app_c/demo/pi_common/tween/tween.vue.wcss",
            _$cssHash = 1437344941;
        TweenDemo["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: tween_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.addField(TweenDemo, ['closePage']);
});