_$pi.define("app_c/chat/chat.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "./msg_item.vue"], function (require, exports, module, direct_1, msg_item_vue_1) {
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
            direct_1.extendAttr(t.n2, w, 4116192616, true);
            t.n3 = direct_1.createDiv();
            direct_1.setEvent(t.n3, "pointerclick", $event => {
                let r = w.goBack();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n3, 2068821430);
            t.n4 = direct_1.createSpan();
            direct_1.setClass(t.n4, 2633031466);
            direct_1.setText(t.n4, "返回");
            t.n5 = direct_1.createDiv();
            direct_1.setAttr(t.n5, "scroll_type", "none");
            direct_1.setAttr(t.n5, "layout", "scroll");
            direct_1.setAttr(t.n5, "scroll_path", "y");
            direct_1.setAttr(t.n5, "id", "list");
            direct_1.setClass(t.n5, 3880933146);
            t.n6 = direct_1.createDiv();
            direct_1.setClass(t.n6, 4162737027);
            t.n7 = direct_1.createFor(w, w.friendLogs, B7);
            t.n9 = direct_1.createDiv();
            direct_1.setStyle(t.n9, 17 /*marginTop*/, staticObj[0]);
            t.n10 = direct_1.createSpan();
            direct_1.setClass(t.n10, 2633031466);
            direct_1.setText(t.n10, "对方rid");
            t.n11 = direct_1.createDiv();
            direct_1.setClass(t.n11, 559891232);
            t.n12 = direct_1.createInput();
            direct_1.setAttr(t.n12, "type", "text");
            direct_1.setAttr(t.n12, "placeholder", "对方rid");
            direct_1.setAttr(t.n12, "value", w.rid);
            direct_1.setEvent(t.n12, "change", $event => {
                let r = w.inputRid($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n12, 3956988137);
            t.n13 = direct_1.createDiv();
            direct_1.setStyle(t.n13, 17 /*marginTop*/, staticObj[0]);
            t.n14 = direct_1.createSpan();
            direct_1.setClass(t.n14, 2633031466);
            direct_1.setText(t.n14, "消息内容");
            t.n15 = direct_1.createDiv();
            direct_1.setClass(t.n15, 559891232);
            t.n16 = direct_1.createInput();
            direct_1.setAttr(t.n16, "type", "text");
            direct_1.setAttr(t.n16, "placeholder", "输入内容");
            direct_1.setAttr(t.n16, "value", w.msg);
            direct_1.setEvent(t.n16, "change", $event => {
                let r = w.inputMsg($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n16, 3956988137);
            t.n17 = direct_1.createDiv();
            t.n18 = direct_1.createSpan();
            direct_1.setEvent(t.n18, "pointerclick", $event => {
                let r = w.sendMsg();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n18, 1269297540);
            direct_1.setText(t.n18, "发送");
            t.n19 = direct_1.createSpan();
            direct_1.setEvent(t.n19, "pointerclick", $event => {
                let r = w.getHistory();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n19, 1269297540);
            direct_1.setText(t.n19, "历史20条");
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n7);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n4, t.n3);
            direct_1.append(t.n3, t.n2);
            t.n7.m(t.n6);
            direct_1.append(t.n6, t.n5);
            direct_1.append(t.n5, t.n2);
            direct_1.append(t.n10, t.n9);
            direct_1.append(t.n9, t.n2);
            direct_1.append(t.n12, t.n11);
            direct_1.append(t.n11, t.n2);
            direct_1.append(t.n14, t.n13);
            direct_1.append(t.n13, t.n2);
            direct_1.append(t.n16, t.n15);
            direct_1.append(t.n15, t.n2);
            direct_1.append(t.n18, t.n17);
            direct_1.append(t.n19, t.n17);
            direct_1.append(t.n17, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w, 4116192616);
            direct_1.patchFor(w, t.n7, w.friendLogs, B7);
            if (dirty0 & 2) direct_1.setAttr(t.n12, "value", w.rid);
            if (dirty0 & 4) direct_1.setAttr(t.n16, "value", w.msg);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class B7 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [item] = t._$ctx;
            t.n8 = direct_1.createWidget(msg_item_vue_1.default, w, {
                attrs: {
                    msgLog: item,
                    key: item.key
                }
            });
            return this.n8;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n8);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n8, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let [item] = t._$ctx;
            let { dirty0 } = w._$info;
            var _$wAttrs = {};
            {
                const _$attrs = {};
                _$attrs.msgLog = item;
                _$attrs.key = item.key;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n8, _$wAttrs);
            return this.n8;
        }
        s(item) {
            let w = this.w;
            this._$ctx[0] = item;
        }
    }
});