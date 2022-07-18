_$pi.define("app_c/demo/ui/show/base/canvas.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./canvas.vue.tpl", "pi_gui/widget/direct", "pi_gui/engine/document"], function (require, exports, module, direct_1, canvas_vue_tpl_1, direct_2, document_1) {
    "use strict";

    exports.initMeta = void 0;
    ;
    let gui1, gui2;
    class CanvasWidget extends direct_2.WidgetBase {
        drawByGui1() {
            gui1.applyStyle(gui1.body.style, "flexWrap", "wrap");
            let node = this.node1 = gui1.createElement("div");
            gui1.applyStyle(node.style, "width", "30px");
            gui1.applyStyle(node.style, "height", "30px");
            gui1.applyStyle(node.style, "backgroundColor", "#ffff55");
            gui1.applyStyle(node.style, "margin", "5px");
            gui1.body.appendChild(node);
        }
        drawByGui2() {
            gui2.applyStyle(gui2.body.style, "flexWrap", "wrap");
            let node = this.node2 = gui2.createElement("div");
            gui2.applyStyle(node.style, "width", "30px");
            gui2.applyStyle(node.style, "height", "30px");
            gui2.applyStyle(node.style, "backgroundColor", "#ffff55");
            gui2.applyStyle(node.style, "margin", "5px");
            gui2.body.appendChild(node);
        }
        deleteCanvas1() {
            this.isDeleteCanvas1 = true;
        }
        deleteCanvas2() {
            this.isDeleteCanvas2 = true;
        }
        firstPaint() {
            let e1 = this.ref("canvas1");
            let context1 = e1.glContext;
            if (!context1.fbo) {
                console.log("canvas context is not exist");
            }
            // 将gui的渲染目标绑定为fbo
            this.gui1 = new document_1.Document(300, 300, context1.engine, { targetX: context1.availRect.x, targetY: context1.availRect.y, viewPortWidth: context1.availRect.width, viewPortHeight: context1.availRect.height, class_sheet: direct_2.document(this).getClassSheet(), font_sheet: direct_2.document(this).getFontSheet() });
            this.gui1.bindRenderTarget(context1.fbo);
            this.gui1.setClearColor({ r: 0, b: 0, g: 0, a: 0 });
            window["canvas_document1"] = this.gui1;
            this.gui1.isNew = true;
            let e2 = this.ref("canvas2");
            let context2 = e2.glContext;
            if (!context2.fbo) {
                console.log("canvas context is not exist");
            }
            // 将gui的渲染目标绑定为fbo
            this.gui2 = new document_1.Document(300, 400, context2.engine, { targetX: context2.availRect.x, targetY: context2.availRect.y, viewPortWidth: context2.availRect.width, viewPortHeight: context2.availRect.height, class_sheet: direct_2.document(this).getClassSheet(), font_sheet: direct_2.document(this).getFontSheet() });
            this.gui2.bindRenderTarget(context2.fbo);
            this.gui2.setClearColor({ r: 0, b: 0, g: 0, a: 0 });
            window["canvas_document2"] = this.gui2;
            this.gui2.isNew = true;
            e1.requestAnimationFram(() => {
                this.gui1.render();
                return true;
            });
            e2.requestAnimationFram(() => {
                this.gui2.render();
                return true;
            });
        }
    }
    exports.default = CanvasWidget;
    exports.initMeta = () => {
        let _$tpl = "app_c/demo/ui/show/base/canvas.vue.tpl.ts",
            _$cssPath = "app_c/demo/ui/show/base/canvas.vue.wcss",
            _$cssHash = 960589919;
        CanvasWidget["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: canvas_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(CanvasWidget, ["isDeleteCanvas1", "isDeleteCanvas2"]);
    direct_1.addField(CanvasWidget, ['node1', 'node2']);
});