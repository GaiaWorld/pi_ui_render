_$pi.define("app_a/widget/cloud/cloud.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./cloud.vue.tpl", "pi_common/ui/main_root", "pi_gui/widget/direct"], function (require, exports, module, direct_1, cloud_vue_tpl_1, main_root_1, direct_2) {
    "use strict";

    exports.initMeta = exports.closeCloud = exports.openCloud = void 0;
    let w;
    class Cloud {
        constructor() {
            this.openTime = 0;
        }
        create() {
            w = this;
        }
        attach() {
            this.openTime = +new Date();
            this.open();
        }
        open() {
            const cloudBox = direct_2.findElementByAttr(this, 'id', 'cloudBox');
            const firstChild = cloudBox.firstChild;
            const secondChild = firstChild.nextSibling;
            this.enterCb && firstChild.style.addAnimListener('l1_0', 'end', () => {
                this.enterCb && this.enterCb();
            });
            firstChild.style.addAnimation(direct_2.createRunTimeAnimation(`l1_0 0.5s linear 0s 1 normal none`, this)[0]);
            secondChild.style.addAnimation(direct_2.createRunTimeAnimation(`r1_0 0.5s linear 0s 1 normal none`, this)[0]);
            this.openTime = Date.now();
        }
        close(cb) {
            const now = Date.now();
            if (now - this.openTime <= 1000) return setTimeout(() => this.close(), 500);
            const cloudBox = direct_2.findElementByAttr(this, 'id', 'cloudBox');
            const firstChild = cloudBox.firstChild;
            const secondChild = firstChild.nextSibling;
            firstChild.style.addAnimation(direct_2.createRunTimeAnimation(`l1_1 0.8s linear 0s 1 normal none`, this)[0]);
            secondChild.style.addAnimation(direct_2.createRunTimeAnimation(`r1_1 0.8s linear 0s 1 normal none`, this)[0]);
            setTimeout(() => {
                w = undefined;
                this.ok && this.ok();
                cb && cb();
            }, 1000);
        }
    }
    exports.default = Cloud;
    exports.openCloud = (data = {}) => {
        if (w) return;
        main_root_1.pop(Cloud, data);
    };
    exports.closeCloud = cb => {
        if (!w) return;
        w.close(cb);
    };
    exports.initMeta = () => {
        let _$tpl = "app_a/widget/cloud/cloud.vue.tpl.ts",
            _$cssPath = "app_a/widget/cloud/cloud.vue.wcss",
            _$cssHash = 1072940825;
        Cloud["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: cloud_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.addField(Cloud, ['openTime', 'ok', 'enterCb']);
});