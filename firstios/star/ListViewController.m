//
//  ViewController.m
//  star
//
//  Created by LavenderUni on 16/6/8.
//  Copyright © 2016年 LavenderUni. All rights reserved.
//

#import "ListViewController.h"
#import "ImgTableViewCell.h"
#import "DetailViewController.h"
#import <SimpleAppLibs/SAHeader.h>


@interface ListViewController () <UITableViewDelegate, UITableViewDataSource>
{
    NSMutableArray *items;
}
@property (weak, nonatomic) IBOutlet UITableView *tableView;

@end

@implementation ListViewController

- (void)viewDidLoad {
    [super viewDidLoad];
    [self.tableView setDelegate:self];
    [self.tableView setDataSource:self];
    
    items = [[NSMutableArray alloc] init];
    [self.tableView reloadData];
    self.tableView.mj_header = [MJRefreshNormalHeader headerWithRefreshingBlock:^{
        [self requestDatas];
    }];

    [self.tableView.mj_header beginRefreshing];
    // Do any additional setup after loading the view, typically from a nib.
}

- (void)requestDatas {
    NSMutableDictionary *params = [[NSMutableDictionary alloc] init];
    [params setValue:@"100" forKey:@"skip"];
    [params setValue:[NSString stringWithFormat:@"%d", 5] forKey:@"limit"];
    
    [[AFNetwork shareManager] requestURL:@"http://service.picasso.adesk.com/v2/homepage"
                                  params:params
                                 success:^(NSURLSessionDataTask *task, NSDictionary *dict){
                                     PLog(@"success");
                                     
                                     [items removeAllObjects];
                                     [self.tableView reloadData];
                                     
                                     if(!dict){
                                         [self.view makeToast:@"dict is null"];
                                         return;
                                     }
                                     NSDictionary *res = [dict valueForKey:@"res"];
                                     if(!res){
                                         [self.view makeToast:@"res is null"];
                                         return;
                                     }
                                     NSArray *wallArray = [res valueForKey:@"wallpaper"];
                                     if (!wallArray || [wallArray count] == 0){
                                         [self.view makeToast:@"wall array is null"];
                                         return;
                                     }
                                     for(NSDictionary *temp in wallArray){
                                         NSString *imageURL = [temp valueForKey:@"img"];
                                         [items addObject:imageURL];
                                     }
                                     [self.tableView reloadData];
                                 }
                                 failure:^(NSURLSessionDataTask *task, NSError *error){
                                     PLog(@"error");
                                 }
                                  finish:^{
                                      [self.tableView.mj_header endRefreshing];
                                  }
     ];
}

- (NSInteger)tableView:(UITableView *)tableView numberOfRowsInSection:(NSInteger)section {
    return [items count];
}

- (__kindof UITableViewCell *)tableView:(UITableView *)tableView cellForRowAtIndexPath:(NSIndexPath *)indexPath{
    ImgTableViewCell *cell = [tableView dequeueReusableCellWithIdentifier:@"ImgTableViewCell" forIndexPath:indexPath];
    if(indexPath.row < items.count){
        NSString *imageURL = [items objectAtIndex:indexPath.row];
        [cell.imageView sd_setImageWithURL:[NSURL URLWithString:imageURL]];
//        [cell.imageV sd_setImageWithURL:[NSURL URLWithString:imageURL]];
    }
    return cell;
}

- (void)tableView:(UITableView *)tableView didSelectRowAtIndexPath:(NSIndexPath *)indexPath{
    DetailViewController *ctrl = [[self storyboard] instantiateViewControllerWithIdentifier:NSStringFromClass([DetailViewController class])];
    NSString *url = [items objectAtIndex:indexPath.row];
    ctrl.urlString = url;
    [self.navigationController pushViewController:ctrl animated:YES];
}

- (void)didReceiveMemoryWarning {
    [super didReceiveMemoryWarning];
    // Dispose of any resources that can be recreated.
}

@end
