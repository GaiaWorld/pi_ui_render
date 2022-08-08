_$pi.define("app_c/demo/ui/show/component/hover_cmd.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[1, 1], [1, 0, 0, 1], [0, 100], [0, 1, 0, 1], [0, 0]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setStyle(t.n2, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n2, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n2, 22 /*flexWrap*/, 1);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createDiv();
            direct_1.setStyle(t.n3, 46 /*backgroundColor*/, staticObj[1]);
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n3, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n3, 29 /*justifyContent*/, 2);
            direct_1.setStyle(t.n3, 28 /*alignItems*/, 2);
            t.n4 = direct_1.createSpan();
            direct_1.setStyle(t.n4, 6 /*position*/, 1);
            direct_1.setStyle(t.n4, 42 /*fontSize*/, 22);
            direct_1.setText(t.n4, w.tip1 ? w.tip1 : "pc鼠标移入触发hover，移动设备点击触发hover");
            t.n5 = direct_1.createDiv();
            direct_1.setStyle(t.n5, 46 /*backgroundColor*/, staticObj[3]);
            direct_1.setStyle(t.n5, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n5, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n5, 29 /*justifyContent*/, 2);
            direct_1.setStyle(t.n5, 28 /*alignItems*/, 2);
            direct_1.setStyle(t.n5, 9 /*bottom*/, staticObj[4]);
            direct_1.setStyle(t.n5, 6 /*position*/, 1);
            direct_1.setEvent(t.n5, "pointerclick", $event => {
                let r = 1 + 2;
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            t.n6 = direct_1.createSpan();
            direct_1.setStyle(t.n6, 6 /*position*/, 1);
            direct_1.setStyle(t.n6, 42 /*fontSize*/, 22);
            direct_1.setText(t.n6, w.tip2 ? w.tip2 : "pc鼠标移入触发hover，移动设备长按触发hover");
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.deleteCmd(t.c3_myhover);
            direct_1.deleteCmd(t.c5_myhover);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n4, t.n3);
            direct_1.append(t.n3, t.n2);
            t.c3_myhover = direct_1.addCmd(w, 'myhover', t.n3, {
                args: 'myhover事件',
                emit: w.emitHover1,
                cancel: w.cancelHover1
            });
            direct_1.append(t.n6, t.n5);
            direct_1.append(t.n5, t.n2);
            t.c5_myhover = direct_1.addCmd(w, 'myhover', t.n5, {
                args: 'myhover事件',
                emit: w.emitHover2,
                cancel: w.cancelHover2
            });
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w);
            if (dirty0 & 4) direct_1.setText(t.n4, w.tip1 ? w.tip1 : "pc鼠标移入触发hover，移动设备点击触发hover");
            if (dirty0 & 32) direct_1.setText(t.n6, w.tip2 ? w.tip2 : "pc鼠标移入触发hover，移动设备长按触发hover");
            return this.n2;
        }
    }
    exports.BW2 = BW2;
});