_$pi.define("app_a/login/client/login.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "app_a/widget/btn/btn.vue", "pi_gui/ui/imgmap.vue"], function (require, exports, module, direct_1, btn_vue_1, imgmap_vue_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[4, 0.17, 0.57, 0.88, 1], [0, 150], [1, 1]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        s17() {
            let w = this.w;
            return w.loginType === '3' ? B17 : null;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.extendAttr(t.n2, w, 2639313270, true);
            t.n3 = direct_1.createDiv();
            direct_1.setClass(t.n3, 112626362);
            t.n4 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "login_bg"
                },
                class: 3165071837
            });
            t.n5 = direct_1.createDiv();
            direct_1.setClass(t.n5, 123895582);
            t.n6 = direct_1.createDiv();
            direct_1.setEvent(t.n6, "pointerclick", $event => {
                let r = w.switchLoginType('3');
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n6, direct_1.classHash(w.loginType === '3' ? 'login_tab_item_active' : 'login_tab_item', w));
            t.n7 = direct_1.createSpan();
            direct_1.setClass(t.n7, 2813476411);
            direct_1.setText(t.n7, "手机号登录");
            t.n8 = direct_1.createDiv();
            direct_1.setEvent(t.n8, "pointerclick", $event => {
                let r = w.switchLoginType('2');
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n8, direct_1.classHash(w.loginType === '3' ? 'login_tab_item' : 'login_tab_item_active', w));
            t.n9 = direct_1.createSpan();
            direct_1.setClass(t.n9, 2813476411);
            direct_1.setText(t.n9, "账号密码登录");
            t.n10 = direct_1.createDiv();
            direct_1.setClass(t.n10, 870522192);
            t.n11 = direct_1.createDiv();
            direct_1.setClass(t.n11, 3502683816);
            t.n12 = direct_1.createDiv();
            direct_1.setClass(t.n12, 2829921613);
            t.n13 = direct_1.createSpan();
            direct_1.setClass(t.n13, 3664070929);
            direct_1.setText(t.n13, w.loginType === '3' ? '手机号' : '账号');
            t.n14 = direct_1.createDiv();
            direct_1.setClass(t.n14, 695472355);
            t.n15 = direct_1.createInput();
            direct_1.setAttr(t.n15, "value", w.input1);
            direct_1.setEvent(t.n15, "change", $event => {
                let r = w.input1Change($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n15, 3069761967);
            t.n16 = direct_1.createDiv();
            direct_1.setEvent(t.n16, "pointerclick", $event => {
                let r = w.getVerifyCode($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n16, 590249799);
            t.i17 = t.s17();
            t.n17 = direct_1.createIf(w, t.i17);
            t.n18 = direct_1.createDiv();
            direct_1.setClass(t.n18, 3502683816);
            t.n19 = direct_1.createDiv();
            direct_1.setClass(t.n19, 2829921613);
            t.n20 = direct_1.createSpan();
            direct_1.setClass(t.n20, 3664070929);
            direct_1.setText(t.n20, w.loginType === '3' ? '验证码' : '密码');
            t.n21 = direct_1.createDiv();
            direct_1.setClass(t.n21, 695472355);
            t.n22 = direct_1.createInput();
            direct_1.setAttr(t.n22, "value", w.input2);
            direct_1.setEvent(t.n22, "change", $event => {
                let r = w.input2Change($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n22, 3069761967);
            t.n23 = direct_1.createDiv();
            direct_1.setAttr(t.n23, "id", "loginBtn");
            direct_1.setEvent(t.n23, "pointerdown", $event => {
                let r = w.down('loginBtn');
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setEvent(t.n23, "pointerup", $event => {
                let r = w.up('loginBtn');
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setEvent(t.n23, "pointerclick", $event => {
                let r = w.handleLogin($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n23, 1054646161);
            t.n24 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "login_btn"
                },
                style: {
                    position /*position*/: "absolute",
                    width /*width*/: "100%",
                    height /*height*/: "100%"
                },
                class: 3905190983
            });
            t.n25 = direct_1.createDiv();
            direct_1.setClass(t.n25, 2980692025);
            t.n26 = direct_1.createSpan();
            direct_1.setStyle(t.n26, 36 /*textShadow*/, "#008bb2 2px 4px 0px");
            direct_1.setStyle(t.n26, 37 /*textStroke*/, staticObj[0]);
            direct_1.setClass(t.n26, 516076718);
            direct_1.setText(t.n26, "点击登录");
            t.n27 = direct_1.createDiv();
            direct_1.setClass(t.n27, 2135417188);
            t.n28 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "注册"
                },
                style: {
                    width /*width*/: "150px",
                    height /*height*/: "70px"
                },
                events: {
                    "ev-click": $event => {
                        let r = w.registerAccount($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n29 = direct_1.createDiv();
            direct_1.setStyle(t.n29, 6 /*position*/, 1);
            direct_1.setStyle(t.n29, 9 /*bottom*/, staticObj[1]);
            direct_1.setStyle(t.n29, 29 /*justifyContent*/, 2);
            direct_1.setStyle(t.n29, 0 /*width*/, staticObj[2]);
            t.n30 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "游客"
                },
                style: {
                    width /*width*/: "150px",
                    height /*height*/: "70px"
                },
                events: {
                    "ev-click": $event => {
                        let r = w.touristLoginClick($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n31 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "QQ"
                },
                style: {
                    width /*width*/: "150px",
                    height /*height*/: "70px"
                },
                events: {
                    "ev-click": $event => {
                        let r = w.doLogin('5');
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n32 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "微信"
                },
                style: {
                    width /*width*/: "150px",
                    height /*height*/: "70px"
                },
                events: {
                    "ev-click": $event => {
                        let r = w.doLogin('4');
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n33 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "微博"
                },
                style: {
                    width /*width*/: "150px",
                    height /*height*/: "70px"
                },
                events: {
                    "ev-click": $event => {
                        let r = w.doLogin('6');
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n4);
            direct_1.destroyContext(t.n17);
            direct_1.destroyContext(t.n24);
            direct_1.destroyContext(t.n28);
            direct_1.destroyContext(t.n30);
            direct_1.destroyContext(t.n31);
            direct_1.destroyContext(t.n32);
            direct_1.destroyContext(t.n33);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n4, t.n3);
            direct_1.append(t.n7, t.n6);
            direct_1.append(t.n6, t.n5);
            direct_1.append(t.n9, t.n8);
            direct_1.append(t.n8, t.n5);
            direct_1.append(t.n5, t.n3);
            direct_1.append(t.n13, t.n12);
            direct_1.append(t.n12, t.n11);
            direct_1.append(t.n15, t.n14);
            direct_1.append(t.n14, t.n11);
            t.n17.m(t.n16);
            direct_1.append(t.n16, t.n11);
            direct_1.append(t.n11, t.n10);
            direct_1.append(t.n20, t.n19);
            direct_1.append(t.n19, t.n18);
            direct_1.append(t.n22, t.n21);
            direct_1.append(t.n21, t.n18);
            direct_1.append(t.n18, t.n10);
            direct_1.mountChildWidget(t.n24, t.n23);
            direct_1.append(t.n26, t.n25);
            direct_1.append(t.n25, t.n23);
            direct_1.append(t.n23, t.n10);
            direct_1.mountChildWidget(t.n28, t.n27);
            direct_1.append(t.n27, t.n10);
            direct_1.append(t.n10, t.n3);
            direct_1.mountChildWidget(t.n30, t.n29);
            direct_1.mountChildWidget(t.n31, t.n29);
            direct_1.mountChildWidget(t.n32, t.n29);
            direct_1.mountChildWidget(t.n33, t.n29);
            direct_1.append(t.n29, t.n3);
            direct_1.append(t.n3, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w, 2639313270);
            if (dirty0 & 1) direct_1.setClass(t.n6, direct_1.classHash(w.loginType === '3' ? 'login_tab_item_active' : 'login_tab_item', w));
            if (dirty0 & 1) direct_1.setClass(t.n8, direct_1.classHash(w.loginType === '3' ? 'login_tab_item' : 'login_tab_item_active', w));
            if (dirty0 & 1) direct_1.setText(t.n13, w.loginType === '3' ? '手机号' : '账号');
            if (dirty0 & 2) direct_1.setAttr(t.n15, "value", w.input1);
            t.n17 = direct_1.patchIf(w, t.n17, t.i17, t.i17 = t.s17(), t.n16);
            if (dirty0 & 1) direct_1.setText(t.n20, w.loginType === '3' ? '验证码' : '密码');
            if (dirty0 & 8) direct_1.setAttr(t.n22, "value", w.input2);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class B17 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n17 = direct_1.createSpan();
            direct_1.setClass(t.n17, 2185224117);
            direct_1.setText(t.n17, w.countDown ? '重新获取（' + w.countDown + 's）' : '获取验证码');
            return this.n17;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n17, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            if (dirty0 & 4) direct_1.setText(t.n17, w.countDown ? '重新获取（' + w.countDown + 's）' : '获取验证码');
            return this.n17;
        }
    }
});