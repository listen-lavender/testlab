//
//  SAHeader.h
//  Pods
//
//  Created by yunfenghan Ling on 6/2/16.
//
//

#import "PLog.h"
#import "AFNetwork.h"
#import "UIView+Border.h"
#import "UIView+CornerRadius.h"

@import SDWebImage;
@import AFNetworking;
@import MBProgressHUD;
@import Toast;
@import MJRefresh;
@import Masonry;

#define mark - 一些常用方法，常量

// 将16进制的颜色值,透明度，转换为uicolor
#define UIColorFromRGBA(rgbValue, alphaValue) [UIColor colorWithRed:((float)((rgbValue & 0xFF0000) >> 16))/255.0 green:((float)((rgbValue & 0xFF00) >> 8))/255.0 blue:((float)(rgbValue & 0xFF))/255.0 alpha:(float)alphaValue]

#define UIColorFromRGB(rgbValue) UIColorFromRGBA(rgbValue, 1.0)


#define appDelegate ((AppDelegate *)[[UIApplication sharedApplication] delegate])
// 设置尺寸
#define DeviceRect [[UIScreen mainScreen] bounds]
#define SCREEN_SCALE [[UIScreen mainScreen] scale]