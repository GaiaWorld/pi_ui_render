_$pi.define("app_a/login/client/register.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "pi_gui/ui/imgmap.vue", "app_a/widget/btn/btn.vue"], function (require, exports, module, direct_1, imgmap_vue_1, btn_vue_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[0, 10]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.extendAttr(t.n2, w, 1997765814, true);
            t.n3 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "login_bg"
                },
                style: {
                    position /*position*/: "absolute",
                    width /*width*/: "100%",
                    height /*height*/: "100%"
                }
            });
            t.n4 = direct_1.createDiv();
            direct_1.setClass(t.n4, 4259662393);
            t.n5 = direct_1.createDiv();
            direct_1.setClass(t.n5, 4012667661);
            t.n6 = direct_1.createDiv();
            direct_1.setClass(t.n6, 8331943);
            t.n7 = direct_1.createSpan();
            direct_1.setClass(t.n7, 2928751131);
            direct_1.setText(t.n7, "账号");
            t.n8 = direct_1.createDiv();
            direct_1.setClass(t.n8, 769024275);
            t.n9 = direct_1.createInput();
            direct_1.setAttr(t.n9, "value", w.account);
            direct_1.setEvent(t.n9, "change", $event => {
                let r = w.accountChange($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n9, 3301391024);
            t.n10 = direct_1.createDiv();
            direct_1.setClass(t.n10, 4012667661);
            t.n11 = direct_1.createDiv();
            direct_1.setClass(t.n11, 8331943);
            t.n12 = direct_1.createSpan();
            direct_1.setClass(t.n12, 2928751131);
            direct_1.setText(t.n12, "密码");
            t.n13 = direct_1.createDiv();
            direct_1.setClass(t.n13, 769024275);
            t.n14 = direct_1.createInput();
            direct_1.setAttr(t.n14, "value", w.pwd);
            direct_1.setEvent(t.n14, "change", $event => {
                let r = w.pwdChange($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n14, 3301391024);
            t.n15 = direct_1.createDiv();
            direct_1.setStyle(t.n15, 17 /*marginTop*/, staticObj[0]);
            t.n16 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "注册"
                },
                class: 2782806592,
                events: {
                    "ev-click": $event => {
                        let r = w.registerClick($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n17 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "关闭"
                },
                class: 2782806592,
                events: {
                    "ev-click": $event => {
                        let r = w.closeClick($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n3);
            direct_1.destroyContext(t.n16);
            direct_1.destroyContext(t.n17);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n3, t.n2);
            direct_1.append(t.n7, t.n6);
            direct_1.append(t.n6, t.n5);
            direct_1.append(t.n9, t.n8);
            direct_1.append(t.n8, t.n5);
            direct_1.append(t.n5, t.n4);
            direct_1.append(t.n12, t.n11);
            direct_1.append(t.n11, t.n10);
            direct_1.append(t.n14, t.n13);
            direct_1.append(t.n13, t.n10);
            direct_1.append(t.n10, t.n4);
            direct_1.mountChildWidget(t.n16, t.n15);
            direct_1.mountChildWidget(t.n17, t.n15);
            direct_1.append(t.n15, t.n4);
            direct_1.append(t.n4, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w, 1997765814);
            if (dirty0 & 1) direct_1.setAttr(t.n9, "value", w.account);
            if (dirty0 & 2) direct_1.setAttr(t.n14, "value", w.pwd);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
});