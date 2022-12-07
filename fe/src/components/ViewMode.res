@module("axios") @val external axiosGet: string => Promise.t<'a> = "get"

let load_list = () => {

}

@react.component
let make = () =>  {
  <div>{"VIEW"->React.string}</div>
}
