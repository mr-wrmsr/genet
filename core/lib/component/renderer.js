import BaseComponent from './base'
import Script from '../script'
import objpath from 'object-path'
import path from 'path'

export default class RendererComponent extends BaseComponent {
  constructor (comp, dir) {
    super()
    const file = objpath.get(comp, 'main', '')
    if (!file) {
      throw new Error('main field required')
    }
    this.id = objpath.get(comp, 'id', '')
    if (!this.id) {
      throw new Error('id field required')
    }
    this.mainFile = path.resolve(dir, file)
    switch (comp.type) {
      case 'core:renderer:attr':
        this.type = 'attr'
        break
      case 'core:renderer:layer':
        this.type = 'layer'
        break
      default:
        throw new Error(`unknown renderer type: ${comp.type}`)
    }
  }
  async load () {
    const component = await Script.execute(this.mainFile)
    if (this.type === 'attr') {
      this.disposable =
        deplug.session.registerAttrRenderer(this.id, component)
    } else if (this.type === 'layer') {
      this.disposable =
        deplug.session.registerLayerRenderer(this.id, component)
    }
    return true
  }
  async unload () {
    if (this.disposable) {
      this.disposable.dispose()
      this.disposable = null
    }
    return true
  }
}
