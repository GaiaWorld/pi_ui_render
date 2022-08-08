_$pi.define("app_c/demo/ui/show/component/list.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./list.vue.tpl", "pi_gui/widget/direct"], function (require, exports, module, direct_1, list_vue_tpl_1, direct_2) {
    "use strict";

    exports.initMeta = exports.Text = void 0;
    // 数据适配器，给List组件的数据，必须实现getCount、getItem两个接口
    class ArrayAdapter {
        getCount() {
            return this.value.length;
        }
        getItem(i) {
            return this.value[i];
        }
    }
    class TestList {
        constructor() {
            this.loading = false;
            this.finished = false;
            this.headHold = 50;
            this.footHold = 50;
            this.arr = new ArrayAdapter();
            this.addArray(100);
        }
        load() {
            if (!this.loading && !this.finished) {
                setTimeout(() => {
                    this.addArray(100);
                    this.arr = this.arr;
                    this.loading = false;
                    // 测试，数量加载到200个时，设置为完成状态
                    if (this.arr.getCount() === 400) {
                        this.finished = true;
                        this.footHold = 0; // 回弹时，直接回弹到0位置
                    }
                    // 通知加载完成
                    direct_2.emit(direct_2.ref(this, "list"), "ev-loadsuccess", this.finished);
                }, 5000);
                this.loading = true;
            }
        }
        // overflow(e:{type:string, speed: number}) {
        // 	this.refs["rubber"].start(e.speed, e.max);
        // 	// console.log("overflow==================", e)
        // 	if(e.type === "start") {
        // 		;
        // 		setTimeout(() => {
        // 			this.refs["rubber"].end();// 结束橡皮经效果
        // 		}, 3000)
        // 	}
        // }
        addArray(v) {
            if (v <= 0) {
                return;
            }
            if (!this.arr.value) {
                this.arr.value = [];
            }
            let r = this.arr.value,
                oldLen = r.length,
                newLen = oldLen + v;
            for (let i = oldLen; i < newLen; i++) {
                r.push(i);
            }
            return r;
        }
    }
    exports.default = TestList;
    class Text {
        destroy() {
            // console.log("destroy Text");
        }
    }
    exports.Text = Text;
    exports.initMeta = () => {
        let _$tpl = "app_c/demo/ui/show/component/list.vue.tpl.ts",
            _$cssPath = "app_c/demo/ui/show/component/list.vue.wcss",
            _$cssHash = 376405719;
        Text["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: list_vue_tpl_1.BW11, cssHash: _$cssHash };
        TestList["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: list_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(TestList, ["headHold", "footHold", "arr"]);
    direct_1.defineAccessors(Text, ["v"]);
    direct_1.addField(TestList, ['loading', 'finished']);
});