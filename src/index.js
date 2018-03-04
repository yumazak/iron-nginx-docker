import React from 'react'
import ReactDOM from 'react-dom'
import request from 'superagent'

class BBSForm extends React.Component {
  constructor (props) {
    super(props)
    this.state = {
      name: '',
      body: ''
    }
  }

  nameChanged (e) {
    this.setState({name: e.target.value})
  }

  bodyChanged (e) {
    this.setState({body: e.target.value})
  }

  post (e) {
    request
      .get('/api/write')
      .query({
        name: this.state.name,
        body: this.state.body
      })
      .end((err, data) => {
        if (err) {
          console.error(err)
        }
        this.setState({body: ''})
        if (this.props.onPost) {
          this.props.onPost()
        }
      })
  }
  render () {
    return (
      <div style={styles.form}>
        名前:<br />
        <input type='text' value={this.state.name}
          onChange={e => this.nameChanged(e)} /><br />
        本文:<br />
        <input type='text' value={this.state.body} size='60'
          onChange={e => this.bodyChanged(e)} /><br />
        <button onClick={e => this.post()}>発言</button>
      </div>
    )
  }
}

class BBSApp extends React.Component {
  constructor (props) {
    super(props)
    this.state = {
      items: []
    }
  }

  componentWillMount () {
    this.loadLogs()
  }

  loadLogs () {
    console.log('hi')
    request
      .get('/api/getItems')
      .end((err, data) => {
        if (err) {
          console.error(err)
          return
        }
        this.setState({items: data.body.datas})
        console.log(data.body)
      })
  }

  render () {
    if (typeof this.state.items !== 'undefined') {
      var itemsHtml = this.state.items.map(e => (
        <li key={e._id}>{e.name} - {e.body}</li>
      ))
    }
    return (
      <div>
        <h1 style={styles.h1}>掲示板</h1>
        <BBSForm onPost={e => this.loadLogs()} />
        <p style={styles.right}>
          <button onClick={e => this.loadLogs()}>再読み込み</button>
        </p>
        <ul>{itemsHtml}</ul>
      </div>
    )
  }
}

const styles = {
  h1: {
    backgroundColor: 'blue',
    color: 'white',
    fontSize: 24,
    padding: 12
  },
  form: {
    padding: 12,
    border: '1px solid silver',
    backgroundColor: '#F0F0F0'
  },
  right: {
    textAlign: 'right'
  }
}
ReactDOM.render(
  <BBSApp />,
  document.getElementById('root')
)
