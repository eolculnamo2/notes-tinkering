@module("axios") @val external axiosPost: (string, {..}) => Promise.t<'a> = "post"

type new_note = {
  title: string,
  body: string,
  category: string,
}

let handle_add = (note: new_note, username: string) => {
  axiosPost(
    ApiConstants.be_url ++ "/users/" ++ username ++ "/notes",
    {
      "title": note.title,
      "body": note.body,
      "category": note.category,
    },
  )
}

type state = {
  title: string,
  body: string,
  category: string,
}

let state = {
  title: "",
  body: "",
  category: "",
}

let state_to_note = (state: state): new_note => {
  title: state.title,
  body: state.body,
  category: state.category,
}
type actions = TitleUpdated(string) | BodyUpdated(string) | CategoryUpdated(string)

let reducer = (state, action) => {
  switch action {
  | TitleUpdated(t) => {...state, title: t}
  | BodyUpdated(b) => {...state, body: b}
  | CategoryUpdated(c) => {...state, category: c}
  }
}

@react.component
let make = (~on_exit: unit => unit, ~username: string) => {
  let (state, dispatch) = React.useReducer(reducer, state)

  let add_item = () => {
    handle_add(state_to_note(state), username)->Promise.then(() => {
      on_exit()
      Promise.resolve()
    })
  }
  <div>
    <a onClick={_ => on_exit()}> {"Exit"->React.string} </a>
    <h3> {"Add"->React.string} </h3>
    <div> {"Title"->React.string} </div>
    <input
      value={state.title}
      onChange={e => ReactEvent.Form.currentTarget(e)["value"]->TitleUpdated->dispatch}
    />
    <div> {"Body"->React.string} </div>
    <input
      value={state.body}
      onChange={e => ReactEvent.Form.currentTarget(e)["value"]->BodyUpdated->dispatch}
    />
    <div> {"Category"->React.string} </div>
    <input
      value={state.category}
      onChange={e => ReactEvent.Form.currentTarget(e)["value"]->CategoryUpdated->dispatch}
    />
    <button onClick={_ => {
      let _ = add_item()
    }}> {"Submit"->React.string} </button>
  </div>
}
