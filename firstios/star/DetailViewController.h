//
//  DetailViewController.h
//  star
//
//  Created by LavenderUni on 16/6/8.
//  Copyright © 2016年 LavenderUni. All rights reserved.
//

#import <UIKit/UIKit.h>

@interface DetailViewController : UIViewController

@property (retain, nonatomic) NSString *urlString;
@property (weak, nonatomic) IBOutlet UIWebView *webview;

@end
