//! Amazon MWS Orders API - Version 2013-09-01
//!
//! [Documentation](http://docs.developer.amazonservices.com/en_US/orders-2013-09-01/Orders_Overview.html)

/// Returns orders created or updated during a time frame that you specify.
///
/// The ListOrders operation returns a list of orders created or updated during a time frame that you specify. 
/// You define that time frame using the CreatedAfter parameter or the LastUpdatedAfter parameter. 
/// You must use one of these parameters, but not both. You can also apply a range of filtering criteria to narrow the list of orders that is returned. 
/// The ListOrders operation includes order information for each order returned, including AmazonOrderId, OrderStatus, FulfillmentChannel, and LastUpdateDate.
///
/// [Documentation](http://docs.developer.amazonservices.com/en_US/orders-2013-09-01/Orders_ListOrders.html)
pub fn list_orders() { unimplemented!() }

/// Returns the next page of orders using the NextToken parameter.
///
/// The ListOrdersByNextToken operation returns the next page of orders using the NextToken value that was returned
/// by your previous request to either ListOrders or ListOrdersByNextToken. 
/// If NextToken is not returned, there are no more pages to return.
pub fn list_orders_by_next_token() { unimplemented!() }

pub fn get_order() { unimplemented!() }