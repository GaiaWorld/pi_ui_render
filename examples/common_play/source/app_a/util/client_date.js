_$pi.define("app_a/util/client_date", ["require", "exports", "module"], function (require, exports, module) {
    "use strict";

    exports.ClientDate = void 0;
    class ClientDate {
        constructor(time) {
            this.date = new Date(time);
        }
        // 更新时间
        static updateTime(serverTime) {
            const now = Date.now();
            this.diff = now - serverTime;
        }
        // 获取当前时间
        static now() {
            const now = Date.now();
            return now - this.diff;
        }
        // 检查延迟点击  在浏览器上连点需要时间160ms
        static checkClickDelay(delay = 300) {
            const now = Date.now();
            if (ClientDate.lastClickTime + delay > now) return false;
            ClientDate.lastClickTime = now;
            return true;
        }
        /**
         * 时间转化为字符串
         */
        static convertSecondToTimeStr(second) {
            let str = '';
            const hour = Math.floor(second / 3600);
            if (hour < 10) {
                str += '0';
                str += hour.toString();
            } else {
                str += hour.toString();
            }
            str += ':';
            const minute = Math.floor((second - hour * 3600) / 60);
            if (minute < 10) {
                str += '0';
                str += minute.toString();
            } else {
                str += minute.toString();
            }
            str += ':';
            const s = second - hour * 3600 - minute * 60;
            if (s < 10) {
                str += '0';
                str += s.toString();
            } else {
                str += s.toString();
            }
            return str;
        }
        getTimeStr(method) {
            let data = this.date[method]();
            if (method === 'getMonth') data++;
            return data.toString().padStart(2, 0);
        }
        parseTime() {
            return {
                day: [this.date.getFullYear(), this.getTimeStr('getMonth'), this.getTimeStr('getDate')],
                time: [this.getTimeStr('getHours'), this.getTimeStr('getMinutes'), this.getTimeStr('getSeconds')]
            };
        }
    }
    exports.ClientDate = ClientDate;
    ClientDate.OneDay = 86400; // 一天(秒)60 * 60 * 24
    ClientDate.OneHour = 3600; // 1小时(秒)60 * 60
    ClientDate.EightHour = 28800; // 8小时(秒)60 * 60 *8
    ClientDate.OneDayMs = 86400000; // 一天(毫秒)60 * 60 * 24*1000
    ClientDate.EightHourMs = 28800000; // 8小时(毫秒)60 * 60 * 8*1000
    ClientDate.sevenDayMs = 604800000; // 七天(毫秒)60 * 60 * 24*7*1000
    // 时间间隔
    ClientDate.diff = 0;
    ClientDate.lastClickTime = 0;
});