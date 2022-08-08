_$pi.define("app_a/util/open", ["require", "exports", "module"], function (require, exports, module) {
    "use strict";
});
// // ==============================================导入
// import {pop as popNew, pop as popNew} from 'pi_common/ui/main_root';
// import { Widget } from 'pi_gui/widget/widget';
// import { Callback } from 'pi_utils/util/util';
// // ==============================================导出
// export const animKey = Symbol('ANIMATION');
// // 队列弹出
// interface Args {
//     name: string;
//     props?: any;
//     priority?: number;
//     ok?: Callback;
//     cancel?: Callback;
//     process?: Callback;
//     back?: Callback | 'cancel' | 'force' | 'next';
// }
// const popQueue: Args[] = [];
// let canPop: boolean = true;
// const checkWaitPop = () => {
//     if (popQueue.length > 0 && canPop) {
//         canPop = false;
//         const args = popQueue.shift();
//         popNew(args.name, args.props, args.ok, args.cancel, args.process, args.back);
//     }
// };
// /**
//  * @param priority 2 > 1
//  */
// export const popWait = (name: string, props?: any, priority?: number, ok?: Callback, cancel?: Callback, process?: Callback, back?: Callback | 'cancel' | 'force' | 'next') => {
//     const newOk = arg => {
//         ok && ok(arg);
//         canPop = true;
//         checkWaitPop();
//     };
//     const newPriority = priority || 0;
//     popQueue.push({ name, props, priority: newPriority, ok: newOk, cancel, process, back });
//     popQueue.sort((a, b) => b.priority - a.priority);
//     checkWaitPop();
// };
// export const setPopWait = (falg: boolean) => {
//     canPop = falg;
// };
// // // 打开界面
// // export const popNew = (name: string, props?: any, ok?: Callback, cancel?: Callback, process?: Callback, back?: Callback | 'cancel' | 'force' | 'next') => {
// //     const close = popNewPage(name, props, currentOk(ok, name), currentCancel(cancel, name), process, back);
// //     if (!close || !close.widget) return;
// //     popNewOver(close.widget);
// // };
// // // 打开提示
// // export const popTip = (name: string, props?: any, ok?: Callback, cancel?: Callback, process?: Callback, back?: Callback | 'cancel' | 'force' | 'next') => {
// //     const close = popNewPage(name, props, ok, cancel, process, back);
// //     if (!close || !close.widget) return;
// // };
// // // 直接打开界面
// // export const open = (name: string, props?: any): Widget => {
// //     return openPage(name, props);
// // };
// // // 直接销毁界面
// // export const destory = (w: Widget): void => {
// //     destroyPage(w);
// // };
// // 界面缓存
// const cacheWidget: Map<string, number> = new Map();
// const currentOk = (ok: Callback, currentName: string) => {
//     return p => {
//         resetCash(currentName);
//         ok && ok(p);
//     };
// };
// const currentCancel = (cancel: Callback, currentName: string) => {
//     return p => {
//         resetCash(currentName);
//         cancel && cancel(p);
//     };
// };
// const resetCash = (currentName: string) => {
//     const current = cacheWidget.get(currentName);
//     if (!current) {
//         console.warn('----------currentOk异常-----------');
//     } else if (current <= 1) {
//         cacheWidget.delete(currentName);
//         if (cacheWidget.size <= 0) hasCovered = true;
//     } else {
//         cacheWidget.set(currentName, current - 1);
//     }
// };
// const popNewOver = (popWidget: Widget) => {
//     const currentName = popWidget.name;
//     const oldSize = cacheWidget.size;
//     cacheWidget.set(currentName, (cacheWidget.get(currentName) || 0) + 1);
//     if (oldSize <= 0) hasCovered = false;
// };
// export let hasCovered = false;