_$pi.define("app_c/chat/msg_item.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        s3() {
            let w = this.w;
            return w.msgLog ? B3 : null;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.extendAttr(t.n2, w, 1136374155, true);
            t.i3 = t.s3();
            t.n3 = direct_1.createIf(w, t.i3);
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n3);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            t.n3.m(t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w, 1136374155);
            t.n3 = direct_1.patchIf(w, t.n3, t.i3, t.i3 = t.s3(), t.n2);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class B3 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n3 = direct_1.createTemplate();
            t.n4 = direct_1.createSpan();
            direct_1.setClass(t.n4, 1110910321);
            direct_1.setText(t.n4, "发布：" + w.msgLog.send);
            t.n5 = direct_1.createSpan();
            direct_1.setClass(t.n5, 1110910321);
            direct_1.setText(t.n5, "内容：" + w.msgLog.msg);
            t.n6 = direct_1.createSpan();
            direct_1.setClass(t.n6, 1110910321);
            direct_1.setText(t.n6, "时间：" + w.getTime());
            return this.n3;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n4, t.n3);
            direct_1.append(t.n5, t.n3);
            direct_1.append(t.n6, t.n3);
            direct_1.insertBefore(t.n3, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            if (dirty0 & 1) direct_1.setText(t.n4, "发布：" + w.msgLog.send);
            if (dirty0 & 1) direct_1.setText(t.n5, "内容：" + w.msgLog.msg);
            direct_1.setText(t.n6, "时间：" + w.getTime());
            return this.n3;
        }
    }
});