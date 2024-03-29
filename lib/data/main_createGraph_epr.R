### DESCRIPTION:
###   This is designed the create the expected price range graph. 
###   It is more streamlined. 
###   
### CALLED FROM:
###   report.RMD
###   
### CALLS:
###   dds.format.requested_stock
###   dds.foramt.chosen_date
###   get_stock_information
###   voex_graph.create_graph.date_lines
###   
### INPUTS:
###   <requested.stock [character; default = NULL]>         : The stock under investigation
###   <chosen.date [date; default = NULL]>                  : The date under investigation
###   <report [logical; default = TRUE]>                    : If TRUE it returns the graph object
###   
### OUTPUT:
###   EITHER:
###     data.frame
###     EITHER:
###       <graph.object: epr.graph>
###       logical::FALSE
###   



#########################
## MAIN FUNCTION CALLER
#########################
epr_graph <- function(data.stock = NULL, requested.stock, chosen.date, report = TRUE){
  dds.format.requested_stock(requested.stock) -> requested.stock
  dds.format.chosen_date(chosen.date) -> chosen.date
  epr_graph.get_data(data.stock, requested.stock, chosen.date) -> data.stock
  if(dds.null_check(data.stock)){
    return(FALSE)
  } else {
    epr_graph.calculate_iv(data.stock) -> data.stock
    epr_graph.calculate_bands(data.stock) -> data.stock
    if(report){
      epr_graph.create_graph(requested.stock, chosen.date, data.stock) -> graph.object
      if(dds.null_check(graph.object)){
        return(FALSE)
      } else {
        return(graph.object)
      }
    } else {
      return(data.stock)
    }
  }
  
}
epr_graph.get_data <- function(data.stock, requested.stock, chosen.date){
  if(!is.null(data.stock) & dds.verify_stock(requested.stock, data.stock)){
    return(data.stock[which(data.stock[, 'date'] <= lubridate::ymd(chosen.date)), ])
  } else {
    tryCatch(
      expr = {
        get_stock_information(requested.stock, chosen.date) -> data.stock
        return(data.stock[which(data.stock[, 'date'] <= lubridate::ymd(chosen.date)), ])
      }, error  = function(e){
        log_event.new(type = 'error', requested.stock = requested.stock, chosen.date = chosen.date,
                      condition.message = conditionMessage(e), error.code = 1,
                      caller.function = 'epr_graph.get_data', process.numeric = 3, line = 54)
        return(NULL)
      }
    ) 
  }
}
### EPR_GRAPH.CALCULATE_IV
#
#
epr_graph.calculate_iv <- function(data.stock){
  epr_graph.calculate_iv.add_row(data.stock) -> data.stock
  return(epr_graph.calculate_iv.calculate(data.stock))
}
epr_graph.calculate_iv.add_row <- function(data.stock){
  data.stock[nrow(data.stock), 'date'] + 1 -> data.stock[(nrow(data.stock) + 1), 'date']
  return(data.stock)
}
epr_graph.calculate_iv.calculate <- function(data.stock){
  (data.stock[, 'iv30'] * sqrt(1/252) * sqrt(2/3.14)) -> data.stock[, 'weekly.iv']
  return(data.stock)
}
### EPR_GRAPH.CALCULATE_BANDS
#
#
#
epr_graph.calculate_bands <- function(data.stock){
  epr_graph.calculate_bands.mid(data.stock) -> data.stock
  epr_graph.calculate_bands.su(data.stock) -> data.stock
  epr_graph.calculate_bands.mo(data.stock) -> data.stock
  return(as.data.frame(data.stock))
}

epr_graph.calculate_bands.mid <- function(data.stock){
  data.stock %>% mutate(mo = lag(close) + (lag(close)*lag(weekly.iv))) -> data.stock
  return(data.stock)
}

epr_graph.calculate_bands.su <- function(data.stock){
  data.stock %>% mutate(mid = lag(close) + (lag(close) * lag(weekly.iv))*0.25) -> data.stock
  return(data.stock)
}

epr_graph.calculate_bands.mo <- function(data.stock){
  data.stock %>% mutate(su = lag(close) - ((lag(close)*lag(weekly.iv)))/2) -> data.stock
  return(data.stock)
}
### EPR_GRAPH.CREATE_GRAPH
#
#
#
epr_graph.create_graph <- function(requested.stock, chosen.date, data.stock){
  tryCatch(
    expr = {
      epr_graph.create_graph.main(requested.stock, chosen.date, data.stock) -> graph.object
      voex_graph.create_graph.date_lines(data.stock, graph.object) -> graph.object
      return(graph.object)   
    }, error = function(e){
      log_event.new(type = 'error', requested.stock = requested.stock, chosen.date = chosen.date,
                    condition.message = conditionMessage(e), error.code = 1,
                    caller.function = 'epr_graph.create_graph', process.numeric = 3, line = 114)
      return(NULL)
    }
  )
}
epr_graph.create_graph.main <- function(requested.stock, chosen.date, data.stock){
  data.stock %>% 
    filter(date <= chosen.date + 1,
           date >= chosen.date - lubridate::dmonths(3)) %>% 
  ggplot() + 
    geom_line(aes(x = date, y = close), color = '#002a56') +
    geom_line(aes(x = date, y = su), color = '#474a58', linetype = 'longdash') +
    geom_line(aes(x= date, y = mo), color = '#474a58', linetype = 'longdash') +
    theme(panel.background = element_blank(),
          axis.title.x=element_blank(),
          axis.text.x=element_blank()) +
    labs(y = 'Price') -> graph.object
  return(graph.object)
}
#
##
###