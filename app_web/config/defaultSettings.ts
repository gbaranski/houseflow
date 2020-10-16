import { Settings as LayoutSettings } from '@ant-design/pro-layout';

export default {
  navTheme: 'light',
  // 拂晓蓝
  logo: 'icon.svg',
  primaryColor: '#1890ff',
  layout: 'mix',
  contentWidth: 'Fluid',
  fixedHeader: false,
  fixSiderbar: true,
  colorWeak: false,
  menu: {
    locale: true,
  },
  title: 'Houseflow',
  pwa: false,
  iconfontUrl: '',
} as LayoutSettings & {
  pwa: boolean;
};
