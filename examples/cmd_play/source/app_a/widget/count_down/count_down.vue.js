_$pi.define("app_a/widget/count_down/count_down.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./count_down.vue.tpl", "pi_gui/widget/direct", "pi_utils/util/frame_mgr"], function (require, exports, module, direct_1, count_down_vue_tpl_1, direct_2, frame_mgr_1) {
    "use strict";

    exports.initMeta = void 0;
    class CountDown {
        constructor() {
            this.lastUpdateTime = 0;
            this.restTimeStr = '';
            this.loop = () => {
                const nowTime = Date.now();
                if (nowTime - this.lastUpdateTime < 1000) return;
                this.lastUpdateTime = nowTime;
                if (!this.calcTime()) this.stopAnimation();
            };
        }
        propsUpdate() {
            if (this.calcTime()) {
                this.startAnimation();
            }
        }
        stopAnimation() {
            frame_mgr_1.getGlobal().clearPermanent(this.loop);
        }
        startAnimation() {
            frame_mgr_1.getGlobal().setPermanent(this.loop);
        }
        calcTime() {
            const nowTime = Date.now();
            let restTime = this.nextTime - nowTime;
            if (restTime <= 0) {
                restTime = 0;
                this.restTimeStr = '';
                this.stopAnimation();
                direct_2.emit(this, 'ev-timeEnd', {});
                return;
            }
            let hour = Math.floor(restTime / (60 * 60 * 1000));
            if (hour >= 24) {
                this.restTimeStr = `${hour / 24 | 0}天`;
                return false;
            }
            hour = formatTime(hour);
            let min = Math.floor((restTime - hour * 3600000) / (60 * 1000));
            min = formatTime(min);
            let sec = Math.floor((restTime - hour * 3600000 - min * 60000) / 1000);
            sec = formatTime(sec);
            this.restTimeStr = `${hour}:${min}:${sec}`;
            return true;
        }
    }
    exports.default = CountDown;
    function formatTime(time) {
        return time < 10 ? time = `0${time}` : `${time}`;
    }
    exports.initMeta = () => {
        let _$tpl = "app_a/widget/count_down/count_down.vue.tpl.ts",
            _$cssPath = "app_a/widget/count_down/count_down.vue.wcss",
            _$cssHash = 1366552344;
        CountDown["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: count_down_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(CountDown, ["color", "fontSize", "restTimeStr"]);
    direct_1.addField(CountDown, ['lastUpdateTime', 'nextTime', 'loop']);
});