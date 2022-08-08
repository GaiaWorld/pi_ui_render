_$pi.define("app_a/util/tool", ["require", "exports", "module", "./client_date", "pi_utils/math/bigint/biginteger", "pi_sys/index", "pi_utils/util/logger"], function (require, exports, module, client_date_1, bigInt, index_1, logger_1) {
    "use strict";

    exports.uuid = exports.isWXgame = exports.compareObject = exports.toType = exports.clone = exports.filterSensitiveWord = exports.checkSensitiveWord = exports.makeSensitiveMap = exports.numberConvert = exports.numberFloor = exports.bigIntStrUnitConvert = exports.unitConvert = exports.calcTextWidth = exports.arrToMap = exports.cached = exports.dateForm = exports.padding = exports.stroke = exports.tomorrowZero = exports.isSameDay = exports.isSameWeek = exports.matchByCondition = exports.traversal = exports.indexByAttr = exports.fixObjAttrByArr = exports.checkValueInString = exports.limitSort = exports.getObjByAttr = exports.getObjsValueToArr = exports.copy = exports.shallowCopy = exports.condsCheck = exports.calculate = exports.condMap = exports.condValue = exports.checkObjHasValue = exports.getNotNullEl = exports.initObjValue = exports.getTypeof = exports.checkTypeof = void 0;
    const { logV, logD, logI, logW, logE } = logger_1.getLogger(module.name, "app");
    // ===================================================== 导出
    /**
     * @description 检测数据类型
     * @param type Object Array Function Null Number String Undefined
     */
    exports.checkTypeof = (value, itype) => {
        return Object.prototype.toString.call(value) === `[object ${itype}]`;
    };
    /**
     * @description 獲取数据类型
     * @return  Object Array Function Null Number String Undefined
     */
    exports.getTypeof = value => {
        return Object.prototype.toString.call(value).replace('[object ', '').replace(']', '');
    };
    /**
     * @description 初始化某个对象的所有属性值
     */
    exports.initObjValue = (obj, value) => {
        for (const k in obj) {
            obj[k] = value;
        }
    };
    /**
     * @description 获取数组非空元素数量
     */
    exports.getNotNullEl = arr => {
        const r = [],
              len = arr.length;
        for (const k in arr) {
            if (parseInt(k, 10) < len) {
                r.push(arr[k]);
            }
        }
        return r;
    };
    /**
     * 检查对象是否还有某些特定的字段且值满足需求
     * @param obj xx
     * @param arr [key1, value1, key2, value2]
     */
    exports.checkObjHasValue = (obj, arr) => {
        for (let j = 0, leng = arr.length; j < leng; j += 2) {
            if (obj[arr[j]] !== arr[j + 1]) {
                return false;
            }
        }
        return true;
    };
    /**
     * @description 条件变量
     */
    exports.condValue = (obj, cond) => {
        let i, n;
        if (typeof cond === typeof '') {
            return obj[cond];
        }
        n = cond.length;
        for (i = 0; i < n; i++) {
            obj = obj[cond];
            if (obj === undefined) {
                return;
            }
        }
        return obj;
    };
    // 条件判断表
    exports.condMap = {
        '>': (a, b) => {
            return a > b;
        },
        '>=': (a, b) => {
            return a >= b;
        },
        '<': (a, b) => {
            return a < b;
        },
        '=<': (a, b) => {
            return a <= b;
        },
        '!=': (a, b) => {
            return a !== b;
        },
        '===': (a, b) => {
            return a === b;
        }
    };
    exports.calculate = {
        '=': (a, b) => {
            return b;
        },
        '+': (a, b) => {
            return a + b;
        },
        '*': (a, b) => {
            return a * b;
        },
        '/': (a, b) => {
            return a / b;
        },
        '-': (a, b) => {
            return a - b;
        },
        '^': (a, b) => {
            return Math.pow(a, b);
        }
    };
    /**
     * @description 判断对象是否满足条件conds
     * @param obj 需要判断的对象
     * @param conds 条件列表 [["hp",">",0],["camp",1],["or",["type",1],...]]
     */
    exports.condsCheck = (obj, conds) => {
        let i, j, c;
        const and = _c => {
            if (_c.length === 2) {
                return exports.condValue(obj, _c[0]) === _c[1];
            } else {
                return exports.condMap[_c[1]](exports.condValue(obj, _c[0]), _c[2]);
            }
        };
        const or = _c => {
            for (j = _c.length - 1; j > 0; j--) {
                if (and(_c[j])) {
                    return true;
                }
            }
            return false;
        };
        for (i = conds.length - 1; i >= 0; i--) {
            c = conds[i];
            if (c[0] === 'or') {
                if (!or(c)) {
                    return false;
                }
            } else if (!and(c)) {
                return false;
            }
        }
        return true;
    };
    /**
     * @description 浅克隆
     * @param arr 需要跳过克隆的字段
     */
    exports.shallowCopy = (o, arr) => {
        const n = {};
        arr = arr || [];
        for (const k in o) {
            if (arr.indexOf(k) >= 0) {
                continue;
            }
            n[k] = o[k];
        }
        return n;
    };
    /**
     * @description 深度克隆对象
     * @param o xx
     */
    exports.copy = o => {
        const deepClone = obj => {
            const t = typeof obj;
            let newObj;
            if (obj === null || t !== 'object') {
                return obj;
            }
            newObj = obj instanceof Array ? [] : {};
            for (const i in obj) {
                newObj[i] = deepClone(obj[i]);
            }
            newObj.__proto__ = obj.__proto__;
            return newObj;
        };
        return deepClone(o);
    };
    /**
     * @description 获取列表中每个对象的某个属性值，通过数组的形式返回，下标与对象在数组中的下标对应
     */
    exports.getObjsValueToArr = (list, key) => {
        const c = [];
        for (let i = 0, len = list.length; i < len; i++) {
            c.push(list[i][key]);
        }
        return c;
    };
    /**
     * 在列表中获取指定字段的对象
     * @param list json对象数组
     * @param arr key-value组成的数组 [key1, value1, key2, value2]
     */
    exports.getObjByAttr = (list, arr) => {
        for (let i = 0, len = list.length; i < len; i++) {
            if (exports.checkObjHasValue(list[i], arr)) {
                return list[i];
            }
        }
        return null;
    };
    /**
     * @description 按某字段排序
     * @param arr 需要排序的对象列表
     * @param key 用来排序的字段
     * @param downup 升序（1）还是降序（-1）
     */
    exports.limitSort = (arr, key, downup) => {
        arr.sort((a, b) => {
            return (a - b) * downup;
        });
    };
    /**
     * @description 检查一个字符串中是否含有某些字符串
     * @param arr 可能含有的字符串数组
     * @param s 待检查字符串
     */
    exports.checkValueInString = (arr, s) => {
        for (let ii = 0, l = arr.length; ii < l; ii++) {
            if (s.indexOf(arr[ii]) >= 0) {
                return true;
            }
        }
        return false;
    };
    /**
     * @description 修改对象属性
     * @param obj 需要修改的对象
     * @param arr 新的属性值[key1,value1,key2,value2,....]
     */
    exports.fixObjAttrByArr = (obj, arr) => {
        for (let j = 0, leng = arr.length; j < leng; j += 2) {
            obj[arr[j]] = arr[j + 1];
        }
    };
    // 查找指定键值对应元素的位置
    exports.indexByAttr = (arr, key, value) => {
        var i;
        for (i = arr.length - 1; i >= 0; i--) {
            if (arr[i] && arr[i][key] === value) {
                break;
            }
        }
        return i;
    };
    /**
     * @description 从后往前遍历数组， 返回true表示移除该元素， 返回false表示停止遍历
     */
    exports.traversal = (arr, func) => {
        // TODO 用链表实现，可以更简单的处理删除、添加的问题
        if (!arr) return;
        var n = arr.length,
            delIndex = -1,
            i,
            el,
            r;
        for (i = n - 1; i >= 0; i--) {
            el = arr[i];
            if (el) {
                try {
                    r = func(el);
                } catch (ex) {
                    logD("traversal, ex: ", ex, ", el:", el);
                }
                if (r === false) break;
                if (r === true) {
                    arr[i] = undefined;
                    delIndex = i;
                }
            } else {
                delIndex = i;
            }
        }
        if (delIndex >= 0) {
            for (i = delIndex + 1; i < n; ++i) {
                el = arr[i];
                if (el) arr[delIndex++] = el;
            }
            arr.length = delIndex;
        }
    };
    // 为掉落准备的通过条件进行匹配
    exports.matchByCondition = (reverse, conditions, item) => {
        if (conditions[0] === 'or') {
            let result = false;
            for (let i = 1; i < conditions.length; i++) {
                if (item[conditions[i][0]] === conditions[i][1]) {
                    result = true;
                    break;
                }
            }
            if (reverse) result = !result;
            return result;
        } else {
            let result = true;
            for (let i = 0; i < conditions.length; i++) {
                if (item[conditions[i][0]] !== conditions[i][1]) {
                    result = false;
                    break;
                }
            }
            if (reverse) result = !result;
            return result;
        }
    };
    ////////////////////
    // 是同一周
    exports.isSameWeek = (now, old) => {
        // 思路: 因为1970年1月1 是周4   所以（天数+3）/7 取整 就是周数  如果相同就是同一周反之就不是
        // 经过测试,是以星期一作为每周的第一天的
        // 所有的地方都是说+4，但经过验证，+4后周六与周日不是同一周
        return Math.floor((Math.floor((old + client_date_1.ClientDate.EightHour) / client_date_1.ClientDate.OneDay) + 3) / 7) === Math.floor((Math.floor((now + client_date_1.ClientDate.EightHour) / client_date_1.ClientDate.OneDay) + 3) / 7);
    };
    // 是同一天   notice: 时间为秒时,rate必须为1,时间为毫秒时候,需传rate=1000
    exports.isSameDay = (now, old, rate = 1) => {
        const oneDayTime = client_date_1.ClientDate.OneDay * rate;
        const eightHourTime = client_date_1.ClientDate.EightHour * rate;
        return Math.floor((old + eightHourTime) / oneDayTime) === Math.floor((now + eightHourTime) / oneDayTime);
    };
    // 明日0点的时间戳
    exports.tomorrowZero = now => {
        // 通过取整计算后，会比当前时间多8小时，故这里第二天0点的时间戳需要减去8小时
        return Math.floor(now / client_date_1.ClientDate.OneDayMs) * client_date_1.ClientDate.OneDayMs + client_date_1.ClientDate.OneDayMs - client_date_1.ClientDate.EightHourMs;
    };
    // 组装边框
    function stroke(outlineColor = '#000', outlineWidth = 2) {
        return { outlineColor, outlineWidth };
    }
    exports.stroke = stroke;
    // 设置padding
    function padding(padding) {
        const paddingArr = padding.toString().split(' ');
        switch (paddingArr.length) {
            case 1:
                paddingArr[1] = paddingArr[0];
            case 2:
                paddingArr[2] = paddingArr[0];
            case 3:
                paddingArr[3] = paddingArr[1];
            default:
        }
        const keys = ['Top', 'Right', 'Bottom', 'Left'];
        const paddingObj = Object.create(null);
        for (let i = 0; i < 4; i++) {
            if (+paddingArr[i]) {
                paddingObj[`padding${keys[i]}`] = +paddingArr[i];
            }
        }
        return paddingObj;
    }
    exports.padding = padding;
    /**
     * 时间按格式输出
     * @param seconds 时间秒数
     * @param form 返回的格式 "x天x时x分x秒", "x:x:x", "x天x:x:x" ,"x:x"
     * @param full 是否用0填充, 输出"00:00:20"
     */
    function dateForm(seconds, form, full) {
        const key = form.split('x');
        const num = [Math.floor(seconds), client_date_1.ClientDate.OneDay, 3600, 60, 1];
        let res = '';
        while (key.length > 1 && (seconds || full)) {
            const temp = seconds % (key.length > 2 ? num[num.length - 2] : num[0] + 1) / num[num.length - 1]; // num[0]+1 表示取余的结果是原值
            res = temp + key[key.length - 1] + res;
            seconds = seconds - seconds % num[num.length - 2];
            key.pop();
            num.pop();
        }
        return res;
    }
    exports.dateForm = dateForm;
    /**
     * 创建一个纯函数的缓存版本, 不支持返回值含有Map的函数
     * @desc 尽可能只对返回值简单的函数使用此函数
     */
    function cached(fn) {
        const cached = new Map();
        return (...arg) => {
            const key = arg.length <= 1 ? arg[0] : arg.join('');
            let result;
            if (!cached.has(key)) {
                result = fn.apply(null, arg);
                cached.set(key, result);
            }
            const data = result || cached.get(key);
            if (typeof data === 'object' && data !== null) {
                return JSON.parse(JSON.stringify(data));
            }
            return data;
        };
    }
    exports.cached = cached;
    // 数组转map
    function arrToMap(arr, key) {
        const map = new Map();
        arr.forEach(v => map.set(v[key], v));
        return map;
    }
    exports.arrToMap = arrToMap;
    // 根据字数及字体大小简单计算字符总长度
    function calcTextWidth(text, fontSize) {
        let width = 0;
        for (const v of text) {
            if (v === '\n') continue;
            width += fontSize;
            if (!(chinesePunct.test(v) || chineseReg.test(v))) {
                width -= fontSize * 3 / 9;
            }
        }
        return width;
    }
    exports.calcTextWidth = calcTextWidth;
    // 规范数字显示
    exports.unitConvert = count => {
        let r;
        // 这里这么写的原因是需要保留一位小数，使用toFixed的原因是保证在小数点后一位为0时有.0显示
        if (count >= 1000000 * 1000) {
            r = `${((Math.floor(count / (1000000 * 1000) * 10) | 0) / 10).toFixed(1)}b`;
        } else if (count >= 1000000) {
            r = `${((Math.floor(count / 1000000 * 10) | 0) / 10).toFixed(1)}m`;
        } else if (count >= 1000) {
            r = `${((Math.floor(count / 1000 * 10) | 0) / 10).toFixed(1)}k`;
        } else {
            r = count.toString();
        }
        return r;
    };
    // 97 122 a-z ASCII
    const generateUnits = () => {
        const units = ['', 'K', 'M', 'B', 'T'];
        for (let i = 97; i < 105; i++) {
            for (let j = 97; j < 123; j++) {
                units.push(`${String.fromCharCode(i)}${String.fromCharCode(j)}`);
            }
        }
        return units;
    };
    const units = generateUnits();
    // 规范数字显示
    exports.bigIntStrUnitConvert = number => {
        const intNumber = Math.floor(number);
        const bigIntStr = bigInt(intNumber).toString();
        const i = Math.ceil(bigIntStr.length / 3) - 1;
        const s = bigIntStr.length - i * 3;
        const pre = bigIntStr.slice(0, s);
        let end = bigIntStr.slice(s, s + (i > 0 ? 2 : 0));
        if (end === '00' || end === "0" || end == '') {
            end = '';
        } else {
            end = `.${end}`;
        }
        return `${pre}${end}${units[i]}`;
    };
    exports.numberFloor = number => {
        const intNumber = Math.floor(number);
        const newVal = bigInt(intNumber).toArray(10).value;
        let start = newVal.length % 3;
        start === 0 && (start = 3);
        for (let i = start; i < newVal.length; i++) {
            newVal[i] = 0;
        }
        return bigInt(newVal.join(''), 10).valueOf();
    };
    // 超过1000为贯， 超过10000000为万贯
    exports.numberConvert = count => {
        let r;
        if (count >= 10000000) {
            r = `${Math.floor(count / 10000000)}万贯`;
        } else if (count >= 1000) {
            r = `${Math.floor(count / 1000)}贯`;
        } else {
            r = count.toString();
        }
        return r;
    };
    /**
     * 构造敏感词map
     * @param sensitiveWordList 敏感词列表
     */
    exports.makeSensitiveMap = sensitiveWordList => {
        // 构造根节点
        const result = new Map();
        for (const word of sensitiveWordList) {
            let map = result;
            for (let i = 0; i < word.length; i++) {
                // 依次获取字
                const char = word.charAt(i);
                // 判断是否存在
                if (map.get(char)) {
                    // 获取下一层节点
                    map = map.get(char);
                } else {
                    // 将当前节点设置为非结尾节点
                    if (map.get('laster') === true) {
                        map.set('laster', false);
                    }
                    const item = new Map();
                    // 新增节点默认为结尾节点
                    item.set('laster', true);
                    map.set(char, item);
                    map = map.get(char);
                }
            }
        }
        return result;
    };
    /**
     * 检查敏感词是否存在
     * @param sensitiveMap 敏感词map
     * @param txt 待检查数据
     * @param index 起始位置
     */
    exports.checkSensitiveWord = (sensitiveMap, txt, index) => {
        let currentMap = sensitiveMap;
        let flag = false;
        let wordNum = 0; // 记录过滤
        let sensitiveWord = ''; // 记录过滤出来的敏感词
        for (let i = index; i < txt.length; i++) {
            const word = txt.charAt(i);
            currentMap = currentMap.get(word);
            if (currentMap) {
                wordNum++;
                sensitiveWord += word;
                if (currentMap.get('laster') === true) {
                    // 表示已到词的结尾
                    flag = true;
                    break;
                }
            } else {
                break;
            }
        }
        // 两字成词
        if (wordNum < 2) flag = false;
        return { flag, sensitiveWord };
    };
    /**
     * 判断文本中是否存在敏感词
     * @param txt 待检查数据
     * @param sensitiveMap 敏感词map
     */
    exports.filterSensitiveWord = (txt, sensitiveMap) => {
        let matchResult = { flag: false, sensitiveWord: '' };
        // 过滤掉除了中文、英文、数字之外的
        const txtTrim = txt.replace(/[^\u4e00-\u9fa5\u0030-\u0039\u0061-\u007a\u0041-\u005a]+/g, '');
        for (let i = 0; i < txtTrim.length; i++) {
            matchResult = exports.checkSensitiveWord(sensitiveMap, txtTrim, i);
            if (matchResult.flag) {
                logD(`sensitiveWord:${matchResult.sensitiveWord}`);
                break;
            }
        }
        return matchResult;
    };
    /**
     * 深度克隆
     * @param   obj 待克隆的对象
     * @return      生成的对象
     */
    exports.clone = obj => {
        let o;
        if (typeof obj === 'object') {
            if (Array.isArray(obj)) {
                o = [];
                for (const v of obj) {
                    o.push(exports.clone(v));
                }
            } else if (obj instanceof Map) {
                o = new Map();
                obj.forEach((v, k) => o.set(k, exports.clone(v)));
            } else {
                o = {};
                Object.getOwnPropertyNames(obj).map(k => o[k] = exports.clone(obj[k]));
            }
        } else {
            o = obj;
        }
        return o;
    };
    // ===================================================== 本地
    // 中文标点:。 ？ ！ ， 、 ； ： “ ” ‘ ' （ ） 《 》 〈 〉 【 】 『 』 「 」 ﹃ ﹄ 〔 〕 … — ～ ﹏ ￥
    const chinesePunct = /\u3002|\uff1f|\uff01|\uff0c|\u3001|\uff1b|\uff1a|\u201c|\u201d|\u2018|\u2019|\uff08|\uff09|\u300a|\u300b|\u3008|\u3009|\u3010|\u3011|\u300e|\u300f|\u300c|\u300d|\ufe43|\ufe44|\u3014|\u3015|\u2026|\u2014|\uff5e|\ufe4f|\uffe5]/;
    // 中文字符集
    const chineseReg = /[\u4e00-\u9fa5]/;
    // ===================================================== 立即执行
    // 获取类型
    exports.toType = function (obj) {
        return {}.toString.call(obj).match(/\s([a-zA-Z]+)/)[1].toLowerCase();
    };
    const typeHandle = {
        'map': compareMap,
        'object': compareObject,
        'array': compareList,
        'number': compareOriginal,
        'string': compareOriginal,
        'boolean': compareOriginal
    };
    function compareOriginal(v1, v2) {
        return v1 === v2;
    }
    function compareList(arr1, arr2) {
        if (arr1.length !== arr2.length) return false;
        for (let len = arr1.length, i = 0; i < len; i++) {
            let v1 = arr1[i],
                v2 = arr2[i],
                t1 = exports.toType(v1);
            const handle = typeHandle[t1];
            if (!handle(v1, v2)) return false;
        }
        return true;
    }
    function compareMap(map1, map2) {
        let s1 = map1.size,
            s2 = map2.size;
        if (s1 == s2 && s1 == 0) return true;
        if (s1 !== s2) return false;
        for (let [key, v1] of map1) {
            let v2 = map2.get(key),
                t1 = exports.toType(v1);
            const handle = typeHandle[t1];
            if (!handle(v1, v2)) return false;
        }
        return true;
    }
    /**
     * 深度比较两个值是否相等(仅针对本项目)
     * @param obj1
     * @param obj2
     */
    function compareObject(obj1, obj2) {
        let type1 = typeof obj1,
            type2 = typeof obj2;
        if (type1 !== type2) return false;
        let keys = Object.keys(obj1);
        for (let len = keys.length, i = 0; i < len; i++) {
            const key = keys[i];
            let v1 = obj1[key],
                v2 = obj2[key],
                t1 = exports.toType(v1);
            const handle = typeHandle[t1];
            if (!handle(v1, v2)) return false;
        }
        return true;
    }
    exports.compareObject = compareObject;
    exports.isWXgame = index_1.PISYS.Env.get('platform_type') === 'minigame';
    function uuid() {
        var s = [];
        var hexDigits = "0123456789abcdef";
        for (var i = 0; i < 36; i++) {
            s[i] = hexDigits.substr(Math.floor(Math.random() * 0x10), 1);
        }
        s[14] = "4"; // bits 12-15 of the time_hi_and_version field to 0010
        s[19] = hexDigits.substr(s[19] & 0x3 | 0x8, 1); // bits 6-7 of the clock_seq_hi_and_reserved to 01
        s[8] = s[13] = s[18] = s[23] = "-";
        var uuid = s.join("");
        return uuid;
    }
    exports.uuid = uuid;
});